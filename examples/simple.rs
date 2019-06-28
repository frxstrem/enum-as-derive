use enum_as_derive::*;

fn main() {
    let foo = get_foo();

    println!("Foo:");
    println!("as_a = {:?}", foo.as_a());
    println!("is_b = {:?}", foo.is_b());
    println!("as_c = {:?}", foo.as_c());

    let bar = get_bar();
    println!("Bar:");
    println!("as_a = {:?}", bar.as_a());
    println!("is_b = {:?}", bar.is_b());
}

#[derive(EnumAs)]
pub enum Foo {
    A(i32),
    B,
    C(i32),
}

#[derive(EnumAs)]
pub enum Bar<T: Copy> {
    A(T),
    B,
}

fn get_foo() -> Foo {
    Foo::C(123)
}

fn get_bar() -> Bar<i32> {
    Bar::A(90)
}
