extern crate apple;
extern crate foo;

use foo::Foo;
use apple::Apple;

impl Apple for Foo {
    fn eat(&self){
        println!("love apples");
    }
}