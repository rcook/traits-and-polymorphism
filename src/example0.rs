use crate::common::*;

/// `frob_it_static` is effectively a _family_ of functions parameterized
/// by type `T`. To a first approximation, the Rust compiler generates a
/// separate function body for unique `T`. Since Rust does not support
/// inheritance/subtype-style polymorphism, the `frobber` argument of
/// `frob_it_static` instantiated for a concrete type, e.g. `Foo`, will
/// accept values _only_ of exactly type `Foo`. This feels very much like
/// Haskell's [_parametric polymorphism_](https://wiki.haskell.org/Polymorphism).
/// This is similar to [templates](https://en.cppreference.com/w/cpp/language/templates)
/// with the addition of [constraints and concepts](https://en.cppreference.com/w/cpp/language/constraints).
/// In `frob_it_static`, `T: Frobber` is a trait constraint specifying that
/// `T` must implement this constraint.
fn frob_it_static<T: Frobber>(frobber: T) -> String {
    frobber.frob()
}

/// `concat_frobs_static` is, again, a _family_ of functions. It has two
/// arguments, both of type `T`, and is used to illustrate clearly the point
/// describe in the previous comment: `T` is a single concrete type, i.e.
/// `frobber0` and `frobber1` must be of exactly the same concrete type, which
/// must have an implementation of the `Frobber` trait.
fn concat_frobs_static<T: Frobber>(frobber0: T, frobber1: T) -> String {
    format!("{}{}", frobber0.frob(), frobber1.frob())
}

/// `concat_different_frobs_static` is parameterized on two different types
/// that must implement the `Frobber` trait. Thus, in this case, we can pass
/// two different types, or the same type for both `T` and `U`. Side note:
/// I haven't studied code size in much detail, but I would guess that each
/// unique pair of types `(T, U)` must potentially generate a separate function
/// body leading to quadratic code size if every possible type `T` is paired
/// with every possible type `U` (in the absence of any smart code deduplication
/// optimizations).
fn concat_different_frobs_static<T: Frobber, U: Frobber>(frobber0: T, frobber1: U) -> String {
    format!("{}{}", frobber0.frob(), frobber1.frob())
}

pub fn run() {
    assert_eq!(frob_it_static(Foo {}), "[Foo.frob]");
    assert_eq!(frob_it_static(Bar {}), "[Bar.frob]");

    assert_eq!(concat_frobs_static(Foo {}, Foo {}), "[Foo.frob][Foo.frob]");
    assert_eq!(concat_frobs_static(Bar {}, Bar {}), "[Bar.frob][Bar.frob]");
    // The following will not compile as T is resolved to a _specific_ type, i.e. Foo
    //assert_eq!(concat_frobs_static(Foo {}, Bar {}), "[Foo.frob][Bar.frob]")

    assert_eq!(
        concat_different_frobs_static(Foo {}, Foo {}),
        "[Foo.frob][Foo.frob]"
    );
    assert_eq!(
        concat_different_frobs_static(Bar {}, Bar {}),
        "[Bar.frob][Bar.frob]"
    );
    assert_eq!(
        concat_different_frobs_static(Foo {}, Bar {}),
        "[Foo.frob][Bar.frob]"
    )
}
