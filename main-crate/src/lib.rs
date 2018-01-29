extern crate foo;
extern crate bar;
extern crate baz;
extern crate mr_trait;

use foo::Foo;
use bar::Bar;
use baz::Baz;


fn foo<T: Bar>(x: T) {
    x.m4();
}

fn qux<T: Baz + ?Sized>(x: &T) {
    x.m5();
}


fn lib() {
    // Inherant
    Foo::m1();
    // Inherant with receiver
    Foo.m2();
    // // Static
    Foo::m3();
    // // UFCS static
    <Foo as Bar>::m3();
    // // Static with receiver
    Foo.m4();
    // // UFCS static with receiver
    Foo::m4(&Foo);

    //What happens here is that the call directly goes to the null node
    let x: &Baz = &Foo;
    // Dynamic
    x.m5();
    // UFCS dynamic
    Baz::m5(x);
    // // UFCS static
    <Foo as Baz>::m5(&Foo);

    // Static vtable
    foo(Foo);
    // // Dynamic vtable
    qux(x);
}

fn test(){
    mr_trait::overloadable(mr_trait::outer_1::outer_2::outer_2::Outer2);
    mr_trait::overloadable(mr_trait::outer_1::double_outer_1::TwoArgs(2,3));
    mr_trait::overloadable(mr_trait::outer_1::outer_1::OneArg(2));
}