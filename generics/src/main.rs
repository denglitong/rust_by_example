use std::fmt::{Debug, Display};
use std::hash::Hash;

// a generic function takes an argument T for any type
fn foo<T>(arg: T) {}

// a concrete type `A`
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

// `Single` is a concrete type
struct Single(A);

// `SingleGen` is a generic type
struct SingleGen<T>(T);

struct CS; // concrete type
struct GenericVal<T>(T);

impl GenericVal<f32> {} // Specify f32
impl GenericVal<CS> {} // specify CS

// `<T>` must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    // concrete type and explicitly takes `A`
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // A
    let _i32 = SingleGen(6); // i32
    let _char = SingleGen('c'); // char

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic(SGen('c'));
    generic::<char>(SGen('c'));

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());

    let empty = Empty;
    let null = Null;

    // deallocate `empty` and `null`
    // Empty.double_drop<Null> Empty is caller, Null is generic type for trait
    empty.double_drop(null);

    // empty;
    // null;

    // Error! Vec<T> does not implement `Display`
    // let s = BS(vec![1]);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    print_debug_where_clauses(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // the trait `HasArea` is not implemented for `Triangle`
    // println!("Area: {}", area(&_triangle));
}

struct BS<T: Display>(T);

// where clauses
struct WBS<T>(T)
where
    T: Display;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn print_debug_where_clauses<T>(t: &T)
where
    T: Debug,
{
    println!("{:?}", t);
}

// `T` must implement `HasArea`, any type which meets the bound can access `HasArea`'s function
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

// generics bounds, bounding restricts the generic to types that conform to the bounds
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// non-copyable types
struct Empty;
struct Null;

// a trait generic over `T`
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and caller U,
// the caller is the second type specifier
impl<T, U> DoubleDrop<T> for U {
    // this method takes ownership of both passed arguments, deallocate both
    fn double_drop(self, t: T) {}
}

// 具有 Copy trait 的 type，说明其数据存储在栈上，数据赋值给其他变量不会发生所有权的转移（其实是复制给了其他变量）
