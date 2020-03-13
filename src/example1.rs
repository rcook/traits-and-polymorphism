use crate::common::*;

/// The `dyn` prefix is optional here (the compiler will warn if it is absent).
/// Regardless, `dyn` is implied and this is inherently dynamically dispatched.
/// The type of the argument must be a reference specified with `&`. You cannot
/// a `Frobber` by value: `Frobber` is a trait so there is no such thing as a
/// _value of type `Frobber`_. Notice how `frob_it_dynamic` has no type arguments.
/// Therefore, this defines a single _concrete_ function as opposed to a family
/// of functions.
fn frob_it_dynamic(frobber: &dyn Frobber) -> String {
    frobber.frob()
}

/// Again, `concat_frobs_dynamic` is a single concrete function and not a family
/// of functions since it has no type arguments. Its two arguments are both references
/// to objects implementing `Frobber` and both marked with `dyn`. Since they are
/// `dyn` and references, however, they can be of different concrete types.
fn concat_frobs_dynamic(frobber0: &dyn Frobber, frobber1: &dyn Frobber) -> String {
    format!("{}{}", frobber0.frob(), frobber1.frob())
}

pub fn run() {
    assert_eq!(frob_it_dynamic(&Foo {}), "[Foo.frob]");
    assert_eq!(frob_it_dynamic(&Bar {}), "[Bar.frob]");

    assert_eq!(
        concat_frobs_dynamic(&Foo {}, &Foo {}),
        "[Foo.frob][Foo.frob]"
    );
    assert_eq!(
        concat_frobs_dynamic(&Bar {}, &Bar {}),
        "[Bar.frob][Bar.frob]"
    );
    assert_eq!(
        concat_frobs_dynamic(&Foo {}, &Bar {}),
        "[Foo.frob][Bar.frob]"
    );
}
