use enum_as_derive::*;

#[derive(EnumAs)]
pub(crate) enum Foo {
    A(i32),
    B(i32),
    C(i32),
}

fn main() {
    let foo = get_foo();

    println!("a = {:?}", foo.as_a());
    println!("b = {:?}", foo.as_b());
    println!("c = {:?}", foo.as_c());
}

fn get_foo() -> Foo {
    Foo::C(123)
}