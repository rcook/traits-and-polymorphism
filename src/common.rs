pub trait Frobber {
    fn frob(&self) -> String;
}

pub struct Foo {}

impl Frobber for Foo {
    fn frob(&self) -> String {
        String::from("[Foo.frob]")
    }
}

pub struct Bar {}

impl Frobber for Bar {
    fn frob(&self) -> String {
        String::from("[Bar.frob]")
    }
}
