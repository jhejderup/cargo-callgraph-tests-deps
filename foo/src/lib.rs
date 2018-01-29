pub struct Foo;

impl Foo {
    pub fn m1() {
        println!("Hello! m1");
    }

    pub fn m2(&self) {
        println!("Hello! m2");
    }
}