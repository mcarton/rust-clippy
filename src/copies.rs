use rustc::lint::*;
use rustc::middle::ty;
use rustc_front::hir::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use syntax::parse::token::InternedString;
use syntax::util::small_vector::SmallVector;
use utils::{SpanlessEq, SpanlessHash};
use utils::{get_parent_expr, in_macro, span_note_and_lint};

/// **What it does:** This lint checks for consecutive `ifs` with the same condition. This lint is
/// `Warn` by default.
///
/// **Why is this bad?** This is probably a copy & paste error.
///
/// **Known problems:** Hopefully none.
///
/// **Example:** `if a == b { .. } else if a == b { .. }`
declare_lint! {
    pub IFS_SAME_COND,
    Warn,
    "consecutive `ifs` with the same condition"
}

/// **What it does:** This lint checks for `if/else` with the same body as the *then* part and the
/// *else* part. This lint is `Warn` by default.
///
/// **Why is this bad?** This is probably a copy & paste error.
///
/// **Known problems:** Hopefully none.
///
/// **Example:** `if .. { 42 } else { 42 }`
declare_lint! {
    pub IF_SAME_THEN_ELSE,
    Warn,
    "if with the same *then* and *else* blocks"
}

/// **What it does:** This lint checks for `match` with identical arm bodies.
///
/// **Why is this bad?** This is probably a copy & paste error.
///
/// **Known problems:** Hopefully none.
///
/// **Example:**
/// ```rust,ignore
/// match foo {
///     Bar => bar(),
///     Quz => quz(),
///     Baz => bar(), // <= oups
/// }
/// ```
declare_lint! {
    pub MATCH_SAME_ARMS,
    Warn,
    "`match` with identical arm bodies"
}

#[derive(Copy, Clone, Debug)]
pub struct CopyAndPaste;

impl LintPass for CopyAndPaste {
    fn get_lints(&self) -> LintArray {
        lint_array![IFS_SAME_COND, IF_SAME_THEN_ELSE, MATCH_SAME_ARMS]
    }
}

impl LateLintPass for CopyAndPaste {
    fn check_expr(&mut self, cx: &LateContext, expr: &Expr) {
        if !in_macro(cx, expr.span) {
            // skip ifs directly in else, it will be checked in the parent if
            if let Some(&Expr{node: ExprIf(_, _, Some(ref else_expr)), ..}) = get_parent_expr(cx, expr) {
                if else_expr.id == expr.id {
                    return;
                }
            }

            let (conds, blocks) = if_sequence(expr);
            lint_same_then_else(cx, blocks.as_slice());
            lint_same_cond(cx, conds.as_slice());
            lint_match_arms(cx, expr);
        }
    }
}

/// Implementation of `IF_SAME_THEN_ELSE`.
fn lint_same_then_else(cx: &LateContext, blocks: &[&Block]) {
    let hash: &Fn(&&Block) -> u64 = &|block| -> u64 {
        let mut h = SpanlessHash::new(cx);
        h.hash_block(block);
        h.finish()
    };

    let eq: &Fn(&&Block, &&Block) -> bool = &|&lhs, &rhs| -> bool { SpanlessEq::new(cx).eq_block(lhs, rhs) };

    if let Some((i, j)) = search_same(blocks, hash, eq) {
        span_note_and_lint(cx,
                           IF_SAME_THEN_ELSE,
                           j.span,
                           "this `if` has identical blocks",
                           i.span,
                           "same as this");
    }
}

/// Implementation of `IFS_SAME_COND`.
fn lint_same_cond(cx: &LateContext, conds: &[&Expr]) {
    let hash: &Fn(&&Expr) -> u64 = &|expr| -> u64 {
        let mut h = SpanlessHash::new(cx);
        h.hash_expr(expr);
        h.finish()
    };

    let eq: &Fn(&&Expr, &&Expr) -> bool = &|&lhs, &rhs| -> bool { SpanlessEq::new(cx).ignore_fn().eq_expr(lhs, rhs) };

    if let Some((i, j)) = search_same(conds, hash, eq) {
        span_note_and_lint(cx,
                           IFS_SAME_COND,
                           j.span,
                           "this `if` has the same condition as a previous if",
                           i.span,
                           "same as this");
    }
}

/// Implementation if `MATCH_SAME_ARMS`.
fn lint_match_arms(cx: &LateContext, expr: &Expr) {
    let hash = |arm: &Arm| -> u64 {
        let mut h = SpanlessHash::new(cx);
        h.hash_expr(&arm.body);
        h.finish()
    };

    let eq = |lhs: &Arm, rhs: &Arm| -> bool {
        SpanlessEq::new(cx).eq_expr(&lhs.body, &rhs.body) &&
            // all patterns should have the same bindings
            bindings(cx, &lhs.pats[0]) == bindings(cx, &rhs.pats[0])
    };

    if let ExprMatch(_, ref arms, MatchSource::Normal) = expr.node {
        if let Some((i, j)) = search_same(&**arms, hash, eq) {
            span_note_and_lint(cx,
                               MATCH_SAME_ARMS,
                               j.body.span,
                               "this `match` has identical arm bodies",
                               i.body.span,
                               "same as this");
        }
    }
}

/// Return the list of condition expressions and the list of blocks in a sequence of `if/else`.
/// Eg. would return `([a, b], [c, d, e])` for the expression
/// `if a { c } else if b { d } else { e }`.
fn if_sequence(mut expr: &Expr) -> (SmallVector<&Expr>, SmallVector<&Block>) {
    let mut conds = SmallVector::zero();
    let mut blocks = SmallVector::zero();

    while let ExprIf(ref cond, ref then_block, ref else_expr) = expr.node {
        conds.push(&**cond);
        blocks.push(&**then_block);

        if let Some(ref else_expr) = *else_expr {
            expr = else_expr;
        } else {
            break;
        }
    }

    // final `else {..}`
    if !blocks.is_empty() {
        if let ExprBlock(ref block) = expr.node {
            blocks.push(&**block);
        }
    }

    (conds, blocks)
}

/// Return the list of bindings in a pattern.
fn bindings<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, pat: &Pat) -> HashMap<InternedString, ty::Ty<'tcx>> {
    fn bindings_impl<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, pat: &Pat, map: &mut HashMap<InternedString, ty::Ty<'tcx>>) {
        match pat.node {
            PatKind::Box(ref pat) | PatKind::Ref(ref pat, _) => bindings_impl(cx, pat, map),
            PatKind::TupleStruct(_, Some(ref pats)) => {
                for pat in pats {
                    bindings_impl(cx, pat, map);
                }
            }
            PatKind::Ident(_, ref ident, ref as_pat) => {
                if let Entry::Vacant(v) = map.entry(ident.node.name.as_str()) {
                    v.insert(cx.tcx.pat_ty(pat));
                }
                if let Some(ref as_pat) = *as_pat {
                    bindings_impl(cx, as_pat, map);
                }
            }
            PatKind::Struct(_, ref fields, _) => {
                for pat in fields {
                    bindings_impl(cx, &pat.node.pat, map);
                }
            }
            PatKind::Tup(ref fields) => {
                for pat in fields {
                    bindings_impl(cx, pat, map);
                }
            }
            PatKind::Vec(ref lhs, ref mid, ref rhs) => {
                for pat in lhs {
                    bindings_impl(cx, pat, map);
                }
                if let Some(ref mid) = *mid {
                    bindings_impl(cx, mid, map);
                }
                for pat in rhs {
                    bindings_impl(cx, pat, map);
                }
            }
            PatKind::TupleStruct(..) |
            PatKind::Lit(..) |
            PatKind::QPath(..) |
            PatKind::Range(..) |
            PatKind::Wild |
            PatKind::Path(..) => (),
        }
    }

    let mut result = HashMap::new();
    bindings_impl(cx, pat, &mut result);
    result
}

fn search_same<T, Hash, Eq>(exprs: &[T], hash: Hash, eq: Eq) -> Option<(&T, &T)>
    where Hash: Fn(&T) -> u64,
          Eq: Fn(&T, &T) -> bool
{
    // common cases
    if exprs.len() < 2 {
        return None;
    } else if exprs.len() == 2 {
        return if eq(&exprs[0], &exprs[1]) {
            Some((&exprs[0], &exprs[1]))
        } else {
            None
        };
    }

    let mut map: HashMap<_, Vec<&_>> = HashMap::with_capacity(exprs.len());

    for expr in exprs {
        match map.entry(hash(expr)) {
            Entry::Occupied(o) => {
                for o in o.get() {
                    if eq(&o, expr) {
                        return Some((&o, expr));
                    }
                }
            }
            Entry::Vacant(v) => {
                v.insert(vec![expr]);
            }
        }
    }

    None
}
