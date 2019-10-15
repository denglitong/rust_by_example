// a generic function takes an argument T for any type
fn foo<T>(arg: T) {}

// a concrete type `A`
struct A;

// `Single` is a concrete type
struct Single(A);

// `SingleGen` is a generic type
struct SingleGen<T>(T);

fn main() {
    // concrete type and explicitly takes `A`
    let _s = Single(A);

     let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // A
    let _i32 = SingleGen(6); // i32
    let _char = SingleGen('c'); // char
}
