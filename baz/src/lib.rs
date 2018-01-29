extern crate foo;

use foo::Foo;

pub trait Baz {
    fn m5(&self);
}

impl Baz for Foo {
    fn m5(&self) {
        println!("Hello! m5");
    }   
}