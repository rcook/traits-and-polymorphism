use crate::common::*;

/// `concat_frobs_from_vec_static` is another family of functions parameterized
/// on types that have implementations of the `Frobber` trait. This function
/// requires a `Vec<T>`: i.e. a homogeneous (all values of the same type) vector
/// of values of type `T`. Note that this function _consumes_ the vector as it
/// takes it by value.
fn concat_frobs_from_vec_static<T: Frobber>(frobbers: Vec<T>) -> String {
    // Probably not the most efficient implementation!
    frobbers
        .iter()
        .map(|frobber| frobber.frob())
        .collect::<Vec<_>>()
        .join(";")
}

/// `concat_frobs_from_vec_dynamic1` is a single concrete function. It consumes
/// a vector of references to objects implementing `Frobber`. Each element of
/// the vector could potentially be a different type. Therefore, this is a
/// _heterogeneous_ vector.
fn concat_frobs_from_vec_dynamic1(frobbers: Vec<&dyn Frobber>) -> String {
    // Probably not the most efficient implementation!
    frobbers
        .iter()
        .map(|frobber| frobber.frob())
        .collect::<Vec<_>>()
        .join(";")
}

/// `concat_frobs_from_vec_dynamic2`&mdash;another single concrete function&mdash;introduces
/// the concept of a _box_. Instead of a vector of references, this is now a vector
/// of _values_ each of which is a _owning reference_ to an object implementing
/// the `Frobber` trait. Note that object ownership is distinctly different from
/// the case of `concat_frobs_from_vec_dynamic1` which takes a vector of references.
/// Both functions _consume_ the vector (since it's passed by value). In addition,
/// `concat_frobs_from_vec_dynamic2` also consumes the _objects) themselves since
/// they are owned by the vector.
fn concat_frobs_from_vec_dynamic2(frobbers: Vec<Box<dyn Frobber>>) -> String {
    // Probably not the most efficient implementation!
    frobbers
        .iter()
        .map(|frobber| frobber.frob())
        .collect::<Vec<_>>()
        .join(";")
}

pub fn run() {
    assert_eq!(
        concat_frobs_from_vec_static(vec![Foo {}, Foo {}]),
        "[Foo.frob];[Foo.frob]"
    );
    assert_eq!(
        concat_frobs_from_vec_static(vec![Bar {}, Bar {}]),
        "[Bar.frob];[Bar.frob]"
    );
    // The following will not compile as T is resolved to a _specific_ type, i.e. Foo
    //assert_eq!(concat_frobs_from_vec_static(vec![Foo {}, Bar {}]), "[Foo.frob];[Bar.frob]")

    assert_eq!(
        concat_frobs_from_vec_dynamic1(vec![&Foo {}, &Foo {}]),
        "[Foo.frob];[Foo.frob]"
    );
    assert_eq!(
        concat_frobs_from_vec_dynamic1(vec![&Bar {}, &Bar {}]),
        "[Bar.frob];[Bar.frob]"
    );
    assert_eq!(
        concat_frobs_from_vec_dynamic1(vec![&Foo {}, &Bar {}]),
        "[Foo.frob];[Bar.frob]"
    );

    assert_eq!(
        concat_frobs_from_vec_dynamic2(vec![Box::new(Foo {}), Box::new(Foo {})]),
        "[Foo.frob];[Foo.frob]"
    );
    assert_eq!(
        concat_frobs_from_vec_dynamic2(vec![Box::new(Bar {}), Box::new(Bar {})]),
        "[Bar.frob];[Bar.frob]"
    );
    assert_eq!(
        concat_frobs_from_vec_dynamic2(vec![Box::new(Foo {}), Box::new(Bar {})]),
        "[Foo.frob];[Bar.frob]"
    )
}
