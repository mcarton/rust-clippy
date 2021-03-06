#![feature(plugin_registrar, box_syntax)]
#![feature(rustc_private, collections)]
#![feature(iter_arith)]
#![feature(custom_attribute)]
#![allow(unknown_lints)]

// this only exists to allow the "dogfood" integration test to work
#[allow(dead_code)]
#[allow(print_stdout)]
fn main() {
    println!("What are you doing? Don't run clippy as an executable");
}

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;
#[macro_use]
extern crate rustc_front;

// Only for the compile time checking of paths
extern crate core;
extern crate collections;

// for unicode nfc normalization
extern crate unicode_normalization;

// for semver check in attrs.rs
extern crate semver;

// for regex checking
extern crate regex_syntax;

extern crate rustc_plugin;

use rustc_plugin::Registry;

pub mod consts;
#[macro_use]
pub mod utils;

// begin lints modules, do not remove this comment, it’s used in `update_lints`
pub mod approx_const;
pub mod array_indexing;
pub mod attrs;
pub mod bit_mask;
pub mod block_in_if_condition;
pub mod collapsible_if;
pub mod copies;
pub mod cyclomatic_complexity;
pub mod derive;
pub mod drop_ref;
pub mod entry;
pub mod enum_clike;
pub mod enum_glob_use;
pub mod enum_variants;
pub mod eq_op;
pub mod escape;
pub mod eta_reduction;
pub mod format;
pub mod formatting;
pub mod identity_op;
pub mod if_not_else;
pub mod items_after_statements;
pub mod len_zero;
pub mod lifetimes;
pub mod loops;
pub mod map_clone;
pub mod matches;
pub mod methods;
pub mod minmax;
pub mod misc;
pub mod misc_early;
pub mod mut_mut;
pub mod mut_reference;
pub mod mutex_atomic;
pub mod needless_bool;
pub mod needless_features;
pub mod needless_update;
pub mod no_effect;
pub mod open_options;
pub mod panic;
pub mod precedence;
pub mod print;
pub mod ptr_arg;
pub mod ranges;
pub mod regex;
pub mod returns;
pub mod shadow;
pub mod strings;
pub mod swap;
pub mod temporary_assignment;
pub mod transmute;
pub mod types;
pub mod unicode;
pub mod vec;
pub mod zero_div_zero;
// end lints modules, do not remove this comment, it’s used in `update_lints`

mod reexport {
    pub use syntax::ast::{Name, NodeId};
}

#[plugin_registrar]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box types::TypePass);
    reg.register_late_lint_pass(box misc::TopLevelRefPass);
    reg.register_late_lint_pass(box misc::CmpNan);
    reg.register_late_lint_pass(box eq_op::EqOp);
    reg.register_early_lint_pass(box enum_variants::EnumVariantNames);
    reg.register_late_lint_pass(box enum_glob_use::EnumGlobUse);
    reg.register_late_lint_pass(box enum_clike::EnumClikeUnportableVariant);
    reg.register_late_lint_pass(box bit_mask::BitMask);
    reg.register_late_lint_pass(box ptr_arg::PtrArg);
    reg.register_late_lint_pass(box needless_bool::NeedlessBool);
    reg.register_late_lint_pass(box needless_bool::BoolComparison);
    reg.register_late_lint_pass(box approx_const::ApproxConstant);
    reg.register_late_lint_pass(box misc::FloatCmp);
    reg.register_early_lint_pass(box precedence::Precedence);
    reg.register_late_lint_pass(box eta_reduction::EtaPass);
    reg.register_late_lint_pass(box identity_op::IdentityOp);
    reg.register_early_lint_pass(box items_after_statements::ItemsAfterStatemets);
    reg.register_late_lint_pass(box mut_mut::MutMut);
    reg.register_late_lint_pass(box mut_reference::UnnecessaryMutPassed);
    reg.register_late_lint_pass(box len_zero::LenZero);
    reg.register_late_lint_pass(box misc::CmpOwned);
    reg.register_late_lint_pass(box attrs::AttrPass);
    reg.register_late_lint_pass(box collapsible_if::CollapsibleIf);
    reg.register_late_lint_pass(box block_in_if_condition::BlockInIfCondition);
    reg.register_late_lint_pass(box misc::ModuloOne);
    reg.register_late_lint_pass(box unicode::Unicode);
    reg.register_late_lint_pass(box strings::StringAdd);
    reg.register_early_lint_pass(box returns::ReturnPass);
    reg.register_late_lint_pass(box methods::MethodsPass);
    reg.register_late_lint_pass(box shadow::ShadowPass);
    reg.register_late_lint_pass(box types::LetPass);
    reg.register_late_lint_pass(box types::UnitCmp);
    reg.register_late_lint_pass(box loops::LoopsPass);
    reg.register_late_lint_pass(box lifetimes::LifetimePass);
    reg.register_late_lint_pass(box entry::HashMapLint);
    reg.register_late_lint_pass(box ranges::StepByZero);
    reg.register_late_lint_pass(box types::CastPass);
    reg.register_late_lint_pass(box types::TypeComplexityPass);
    reg.register_late_lint_pass(box matches::MatchPass);
    reg.register_late_lint_pass(box misc::PatternPass);
    reg.register_late_lint_pass(box minmax::MinMaxPass);
    reg.register_late_lint_pass(box open_options::NonSensicalOpenOptions);
    reg.register_late_lint_pass(box zero_div_zero::ZeroDivZeroPass);
    reg.register_late_lint_pass(box mutex_atomic::MutexAtomic);
    reg.register_late_lint_pass(box needless_features::NeedlessFeaturesPass);
    reg.register_late_lint_pass(box needless_update::NeedlessUpdatePass);
    reg.register_late_lint_pass(box no_effect::NoEffectPass);
    reg.register_late_lint_pass(box map_clone::MapClonePass);
    reg.register_late_lint_pass(box temporary_assignment::TemporaryAssignmentPass);
    reg.register_late_lint_pass(box transmute::UselessTransmute);
    reg.register_late_lint_pass(box cyclomatic_complexity::CyclomaticComplexity::new(25));
    reg.register_late_lint_pass(box escape::EscapePass);
    reg.register_early_lint_pass(box misc_early::MiscEarly);
    reg.register_late_lint_pass(box misc::UsedUnderscoreBinding);
    reg.register_late_lint_pass(box array_indexing::ArrayIndexing);
    reg.register_late_lint_pass(box panic::PanicPass);
    reg.register_late_lint_pass(box strings::StringLitAsBytes);
    reg.register_late_lint_pass(box derive::Derive);
    reg.register_late_lint_pass(box types::CharLitAsU8);
    reg.register_late_lint_pass(box print::PrintLint);
    reg.register_late_lint_pass(box vec::UselessVec);
    reg.register_late_lint_pass(box drop_ref::DropRefPass);
    reg.register_late_lint_pass(box types::AbsurdExtremeComparisons);
    reg.register_late_lint_pass(box regex::RegexPass::default());
    reg.register_late_lint_pass(box copies::CopyAndPaste);
    reg.register_late_lint_pass(box format::FormatMacLint);
    reg.register_early_lint_pass(box formatting::Formatting);
    reg.register_late_lint_pass(box swap::Swap);
    reg.register_early_lint_pass(box if_not_else::IfNotElse);

    reg.register_lint_group("clippy_pedantic", vec![
        enum_glob_use::ENUM_GLOB_USE,
        matches::SINGLE_MATCH_ELSE,
        methods::OPTION_UNWRAP_USED,
        methods::RESULT_UNWRAP_USED,
        methods::WRONG_PUB_SELF_CONVENTION,
        mut_mut::MUT_MUT,
        mutex_atomic::MUTEX_INTEGER,
        print::PRINT_STDOUT,
        print::USE_DEBUG,
        shadow::SHADOW_REUSE,
        shadow::SHADOW_SAME,
        shadow::SHADOW_UNRELATED,
        strings::STRING_ADD,
        strings::STRING_ADD_ASSIGN,
        types::CAST_POSSIBLE_TRUNCATION,
        types::CAST_POSSIBLE_WRAP,
        types::CAST_PRECISION_LOSS,
        types::CAST_SIGN_LOSS,
        unicode::NON_ASCII_LITERAL,
        unicode::UNICODE_NOT_NFC,
    ]);

    reg.register_lint_group("clippy", vec![
        approx_const::APPROX_CONSTANT,
        array_indexing::OUT_OF_BOUNDS_INDEXING,
        attrs::DEPRECATED_SEMVER,
        attrs::INLINE_ALWAYS,
        bit_mask::BAD_BIT_MASK,
        bit_mask::INEFFECTIVE_BIT_MASK,
        block_in_if_condition::BLOCK_IN_IF_CONDITION_EXPR,
        block_in_if_condition::BLOCK_IN_IF_CONDITION_STMT,
        collapsible_if::COLLAPSIBLE_IF,
        copies::IF_SAME_THEN_ELSE,
        copies::IFS_SAME_COND,
        copies::MATCH_SAME_ARMS,
        cyclomatic_complexity::CYCLOMATIC_COMPLEXITY,
        derive::DERIVE_HASH_XOR_EQ,
        derive::EXPL_IMPL_CLONE_ON_COPY,
        drop_ref::DROP_REF,
        entry::MAP_ENTRY,
        enum_clike::ENUM_CLIKE_UNPORTABLE_VARIANT,
        enum_variants::ENUM_VARIANT_NAMES,
        eq_op::EQ_OP,
        escape::BOXED_LOCAL,
        eta_reduction::REDUNDANT_CLOSURE,
        format::USELESS_FORMAT,
        formatting::SUSPICIOUS_ASSIGNMENT_FORMATTING,
        formatting::SUSPICIOUS_ELSE_FORMATTING,
        identity_op::IDENTITY_OP,
        if_not_else::IF_NOT_ELSE,
        items_after_statements::ITEMS_AFTER_STATEMENTS,
        len_zero::LEN_WITHOUT_IS_EMPTY,
        len_zero::LEN_ZERO,
        lifetimes::NEEDLESS_LIFETIMES,
        lifetimes::UNUSED_LIFETIMES,
        loops::EMPTY_LOOP,
        loops::EXPLICIT_COUNTER_LOOP,
        loops::EXPLICIT_ITER_LOOP,
        loops::FOR_KV_MAP,
        loops::FOR_LOOP_OVER_OPTION,
        loops::FOR_LOOP_OVER_RESULT,
        loops::ITER_NEXT_LOOP,
        loops::NEEDLESS_RANGE_LOOP,
        loops::REVERSE_RANGE_LOOP,
        loops::UNUSED_COLLECT,
        loops::WHILE_LET_LOOP,
        loops::WHILE_LET_ON_ITERATOR,
        map_clone::MAP_CLONE,
        matches::MATCH_BOOL,
        matches::MATCH_OVERLAPPING_ARM,
        matches::MATCH_REF_PATS,
        matches::SINGLE_MATCH,
        methods::CHARS_NEXT_CMP,
        methods::CLONE_DOUBLE_REF,
        methods::CLONE_ON_COPY,
        methods::EXTEND_FROM_SLICE,
        methods::FILTER_NEXT,
        methods::NEW_RET_NO_SELF,
        methods::OK_EXPECT,
        methods::OPTION_MAP_UNWRAP_OR,
        methods::OPTION_MAP_UNWRAP_OR_ELSE,
        methods::OR_FUN_CALL,
        methods::SEARCH_IS_SOME,
        methods::SHOULD_IMPLEMENT_TRAIT,
        methods::SINGLE_CHAR_PATTERN,
        methods::STR_TO_STRING,
        methods::STRING_TO_STRING,
        methods::WRONG_SELF_CONVENTION,
        minmax::MIN_MAX,
        misc::CMP_NAN,
        misc::CMP_OWNED,
        misc::FLOAT_CMP,
        misc::MODULO_ONE,
        misc::REDUNDANT_PATTERN,
        misc::TOPLEVEL_REF_ARG,
        misc::USED_UNDERSCORE_BINDING,
        misc_early::DUPLICATE_UNDERSCORE_ARGUMENT,
        misc_early::REDUNDANT_CLOSURE_CALL,
        misc_early::UNNEEDED_FIELD_PATTERN,
        mut_reference::UNNECESSARY_MUT_PASSED,
        mutex_atomic::MUTEX_ATOMIC,
        needless_bool::BOOL_COMPARISON,
        needless_bool::NEEDLESS_BOOL,
        needless_features::UNSTABLE_AS_MUT_SLICE,
        needless_features::UNSTABLE_AS_SLICE,
        needless_update::NEEDLESS_UPDATE,
        no_effect::NO_EFFECT,
        open_options::NONSENSICAL_OPEN_OPTIONS,
        panic::PANIC_PARAMS,
        precedence::PRECEDENCE,
        ptr_arg::PTR_ARG,
        ranges::RANGE_STEP_BY_ZERO,
        ranges::RANGE_ZIP_WITH_LEN,
        regex::INVALID_REGEX,
        regex::REGEX_MACRO,
        regex::TRIVIAL_REGEX,
        returns::LET_AND_RETURN,
        returns::NEEDLESS_RETURN,
        strings::STRING_LIT_AS_BYTES,
        swap::ALMOST_SWAPPED,
        swap::MANUAL_SWAP,
        temporary_assignment::TEMPORARY_ASSIGNMENT,
        transmute::USELESS_TRANSMUTE,
        types::ABSURD_EXTREME_COMPARISONS,
        types::BOX_VEC,
        types::CHAR_LIT_AS_U8,
        types::LET_UNIT_VALUE,
        types::LINKEDLIST,
        types::TYPE_COMPLEXITY,
        types::UNIT_CMP,
        unicode::ZERO_WIDTH_SPACE,
        vec::USELESS_VEC,
        zero_div_zero::ZERO_DIVIDED_BY_ZERO,
    ]);
}
