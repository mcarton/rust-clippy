#![feature(plugin)]
#![plugin(clippy)]

#![allow(unused)]
#![deny(clippy, clippy_pedantic)]

use std::ops::Mul;

struct T;

impl T {
    fn add(self, other: T) -> T { self } //~ERROR defining a method called `add`
    fn drop(&mut self) { } //~ERROR defining a method called `drop`

    fn sub(&self, other: T) -> &T { self } // no error, self is a ref
    fn div(self) -> T { self } // no error, different #arguments
    fn rem(self, other: T) { } // no error, wrong return type

    fn into_u32(self) -> u32 { 0 } // fine
    fn into_u16(&self) -> u16 { 0 } //~ERROR methods called `into_*` usually take self by value

    fn to_something(self) -> u32 { 0 } //~ERROR methods called `to_*` usually take self by reference
}

#[derive(Clone,Copy)]
struct U;

impl U {
    fn to_something(self) -> u32 { 0 } // ok because U is Copy
}

impl Mul<T> for T {
    type Output = T;
    fn mul(self, other: T) -> T { self } // no error, obviously
}

/// Utility macro to test linting behavior in `option_methods()`
/// The lints included in `option_methods()` should not lint if the call to map is partially
/// within a macro
macro_rules! opt_map {
    ($opt:expr, $map:expr) => {($opt).map($map)};
}

/// Checks implementation of the following lints:
/// OPTION_MAP_UNWRAP_OR
/// OPTION_MAP_UNWRAP_OR_ELSE
fn option_methods() {
    let opt = Some(1);

    // Check OPTION_MAP_UNWRAP_OR
    // single line case
    let _ = opt.map(|x| x + 1) //~  ERROR called `map(f).unwrap_or(a)`
                               //~| NOTE replace `map(|x| x + 1).unwrap_or(0)`
               .unwrap_or(0); // should lint even though this call is on a separate line
    // multi line cases
    let _ = opt.map(|x| { //~ ERROR called `map(f).unwrap_or(a)`
                        x + 1
                    }
              ).unwrap_or(0);
    let _ = opt.map(|x| x + 1) //~ ERROR called `map(f).unwrap_or(a)`
               .unwrap_or({
                    0
                });
    // macro case
    let _ = opt_map!(opt, |x| x + 1).unwrap_or(0); // should not lint

    // Check OPTION_MAP_UNWRAP_OR_ELSE
    // single line case
    let _ = opt.map(|x| x + 1) //~  ERROR called `map(f).unwrap_or_else(g)`
                               //~| NOTE replace `map(|x| x + 1).unwrap_or_else(|| 0)`
               .unwrap_or_else(|| 0); // should lint even though this call is on a separate line
    // multi line cases
    let _ = opt.map(|x| { //~ ERROR called `map(f).unwrap_or_else(g)`
                        x + 1
                    }
              ).unwrap_or_else(|| 0);
    let _ = opt.map(|x| x + 1) //~ ERROR called `map(f).unwrap_or_else(g)`
               .unwrap_or_else(||
                    0
                );
    // macro case
    let _ = opt_map!(opt, |x| x + 1).unwrap_or_else(|| 0); // should not lint

}

/// Struct to generate false positive for Iterator-based lints
#[derive(Copy, Clone)]
struct IteratorFalsePositives {
    foo: u32,
}

impl IteratorFalsePositives {
    fn filter(self) -> IteratorFalsePositives {
        self
    }

    fn next(self) -> IteratorFalsePositives {
        self
    }

    fn find(self) -> Option<u32> {
        Some(self.foo)
    }

    fn position(self) -> Option<u32> {
        Some(self.foo)
    }

    fn rposition(self) -> Option<u32> {
        Some(self.foo)
    }
}

/// Checks implementation of FILTER_NEXT lint
fn filter_next() {
    let v = vec![3, 2, 1, 0, -1, -2, -3];

    // check single-line case
    let _ = v.iter().filter(|&x| *x < 0).next();
    //~^ ERROR called `filter(p).next()` on an Iterator.
    //~| NOTE replace `filter(|&x| *x < 0).next()`

    // check multi-line case
    let _ = v.iter().filter(|&x| { //~ERROR called `filter(p).next()` on an Iterator.
                                *x < 0
                            }
                   ).next();

    // check that we don't lint if the caller is not an Iterator
    let foo = IteratorFalsePositives { foo: 0 };
    let _ = foo.filter().next();
}

/// Checks implementation of SEARCH_IS_SOME lint
fn search_is_some() {
    let v = vec![3, 2, 1, 0, -1, -2, -3];

    // check `find().is_some()`, single-line
    let _ = v.iter().find(|&x| *x < 0).is_some();
    //~^ ERROR called `is_some()` after searching
    //~| NOTE replace `find(|&x| *x < 0).is_some()`

    // check `find().is_some()`, multi-line
    let _ = v.iter().find(|&x| { //~ERROR called `is_some()` after searching
                              *x < 0
                          }
                   ).is_some();

    // check `position().is_some()`, single-line
    let _ = v.iter().position(|&x| x < 0).is_some();
    //~^ ERROR called `is_some()` after searching
    //~| NOTE replace `position(|&x| x < 0).is_some()`

    // check `position().is_some()`, multi-line
    let _ = v.iter().position(|&x| { //~ERROR called `is_some()` after searching
                                  x < 0
                              }
                   ).is_some();

    // check `rposition().is_some()`, single-line
    let _ = v.iter().rposition(|&x| x < 0).is_some();
    //~^ ERROR called `is_some()` after searching
    //~| NOTE replace `rposition(|&x| x < 0).is_some()`

    // check `rposition().is_some()`, multi-line
    let _ = v.iter().rposition(|&x| { //~ERROR called `is_some()` after searching
                                   x < 0
                               }
                   ).is_some();

    // check that we don't lint if the caller is not an Iterator
    let foo = IteratorFalsePositives { foo: 0 };
    let _ = foo.find().is_some();
    let _ = foo.position().is_some();
    let _ = foo.rposition().is_some();
}

/// Checks implementation of the OR_FUN_CALL lint
fn or_fun_call() {
    struct Foo;

    impl Foo {
        fn new() -> Foo { Foo }
    }

    fn make<T>() -> T { unimplemented!(); }

    let with_constructor = Some(vec![1]);
    with_constructor.unwrap_or(make());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_constructor.unwrap_or_else(make)

    let with_new = Some(vec![1]);
    with_new.unwrap_or(Vec::new());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_new.unwrap_or_default();

    let with_const_args = Some(vec![1]);
    with_const_args.unwrap_or(Vec::with_capacity(12));
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_const_args.unwrap_or_else(|| Vec::with_capacity(12));

    let with_err : Result<_, ()> = Ok(vec![1]);
    with_err.unwrap_or(make());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_err.unwrap_or_else(|_| make());

    let with_err_args : Result<_, ()> = Ok(vec![1]);
    with_err_args.unwrap_or(Vec::with_capacity(12));
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_err_args.unwrap_or_else(|_| Vec::with_capacity(12));

    let with_default_trait = Some(1);
    with_default_trait.unwrap_or(Default::default());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_default_trait.unwrap_or_default();

    let with_default_type = Some(1);
    with_default_type.unwrap_or(u64::default());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_default_type.unwrap_or_default();

    let with_vec = Some(vec![1]);
    with_vec.unwrap_or(vec![]);
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION with_vec.unwrap_or_else(|| vec![]);

    let without_default = Some(Foo);
    without_default.unwrap_or(Foo::new());
    //~^ERROR use of `unwrap_or`
    //~|HELP try this
    //~|SUGGESTION without_default.unwrap_or_else(Foo::new);
}

fn main() {
    use std::io;

    let opt = Some(0);
    let _ = opt.unwrap();  //~ERROR used unwrap() on an Option

    let res: Result<i32, ()> = Ok(0);
    let _ = res.unwrap();  //~ERROR used unwrap() on a Result

    let _ = "str".to_string();  //~ERROR `"str".to_owned()` is faster

    let v = &"str";
    let string = v.to_string();  //~ERROR `(*v).to_owned()` is faster
    let _again = string.to_string();  //~ERROR `String.to_string()` is a no-op

    res.ok().expect("disaster!"); //~ERROR called `ok().expect()`
    // the following should not warn, since `expect` isn't implemented unless
    // the error type implements `Debug`
    let res2: Result<i32, MyError> = Ok(0);
    res2.ok().expect("oh noes!");
    // we currently don't warn if the error type has a type parameter
    // (but it would be nice if we did)
    let res3: Result<u32, MyErrorWithParam<u8>>= Ok(0);
    res3.ok().expect("whoof");
    let res4: Result<u32, io::Error> = Ok(0);
    res4.ok().expect("argh"); //~ERROR called `ok().expect()`
    let res5: io::Result<u32> = Ok(0);
    res5.ok().expect("oops"); //~ERROR called `ok().expect()`
    let res6: Result<u32, &str> = Ok(0);
    res6.ok().expect("meh"); //~ERROR called `ok().expect()`
}

struct MyError(()); // doesn't implement Debug

#[derive(Debug)]
struct MyErrorWithParam<T> {
    x: T
}
