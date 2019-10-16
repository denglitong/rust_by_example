use std::fmt::{Debug, Display};
use std::hash::Hash;

use std::marker::PhantomData;

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

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);

    compare_types(&array, &vec);

    let vec = vec![1, 2, 3];
    vec.print_in_option();

    let age = Years(5);
    let age_days = age.to_days();
    println!("old enough {}", old_enough(&age));
    println!("old enough {}", old_enough(&age_days.to_years()));
    //println!("old enough {}", old_enough(&age_days));

    // obtain base type value of newtype value
    let years = Years(42);
    let year_as_primitive = years.0;
    println!("year as primitive: {}", year_as_primitive);

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {} : {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));

    let container = AContainer(number_1, number_2);

    println!(
        "Does container contain {} and {} : {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2),
        // AContains::contains(&container, &number_1, &number_2),
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", a_difference(&container));

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phandom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phandom: PhantomData,
    };

    // compile-time error! type mismatch so these cannot be compared
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2); // expected f32, found f64
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}

// a phantom tuple struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// storage is allocated for generic type `A`, but not for `B`.
// Therefore, `B` cannot be used in computations.

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phandom: PhantomData<B>,
}

// associated types
struct AContainer(i32, i32);

trait AContains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl AContains for AContainer {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        &self.0 == number_1 && &self.1 == number_2
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn a_difference<C: AContains>(container: &C) -> i32 {
    container.last() - container.first()
}

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // explicitly requires `A` and `B`
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        &self.0 == number_1 && &self.1 == number_2
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// here C is generics type which is Contains<A, B>,
// so fn difference need to specify all of generic types,
// though we just need arg Container<A, B> as input type, C
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.first() - container.last()
}

// new type idiom, gives compile time guarantees that the right type of value match
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // we want `Option<T>: Debug` as our bound because that is what's being printed,
    // that is, print needs type Option<T>, doing otherwise would be using the wrong bound.
    // the self arg will take `T` ownership,
    // and `T` can be caller directly to call trait function without explicit implement
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn compare_prints<T>(t: &T)
where
    T: Debug + Display,
{
    println!("Debug: `{:?}`", t);
    println!("Display: `{:?}`", t);
}

fn compare_types<T, U>(t: &T, u: &U)
where
    T: Debug,
    U: Debug,
{
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
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
