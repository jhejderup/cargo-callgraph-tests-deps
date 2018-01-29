extern crate foo;

use foo::Foo;

pub trait Bar {
    fn m3();
    fn m4(&self);
}

impl Bar for Foo {
    fn m3() {
        println!("Hello! m3");
    }

    fn m4(&self) {
        println!("Hello! m4");
    }   
}