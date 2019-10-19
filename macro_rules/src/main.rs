// 宏元编程 macro meta-programming
// Rust中的宏看起来就像函数一样，除了调用的时候结尾跟着一个!
// Rust中的宏是在构建抽象语法树的时候展开的，而不像C语言中的是字符串预处理的时候

// this is a simple macro named `say_hello`
macro_rules! say_hello {
    // () indicates that the macro takes no argument
    () => {
        // the macro will expand into the contents of this block.
        println!("Hello!");
    };
}

// macro is useful:
// 1.DRY: don't repeat yourself
// 2.Domain-specific languages, macros allow you to define special syntax for a specific purpose
// 3.variadic interfaces

// the arg of a macro are prefixed by a dollar sign $ and type annotated with a designator
macro_rules! create_function {
    // this macro takes an argument of type `ident` and creates a function named `$func_name`
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// create functions named `foo` and `bar`
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // the macro takes an expr argument
    ($expression: expr) => {
        println!("{:?} => {:?}", stringify!($expression), $expression);
    };
}

/*
some of available designators:
block
expr: use for expressions
ident: used for variable/function names
item
literal: used for literal constants
pat: pattern
path
stmt: statement
tt: token tree
ty: type
vis: visibility qualifier
*/

// macro can have overload binding
macro_rules! test {
    ($left: expr; and $right: expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };

    ($left: expr; or $right: expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };

    ($left: expr; + $right: expr) => {
        $left + $right
    };
}

macro_rules! find_min {
    // base case, one argument
    ($x: expr) => ($x);
    // $(...),+ will match one or more expression, separated by commas, also note that semicolon
    // is optional on the last case
    ($x: expr, $($y: expr),+) => {
        std::cmp::min($x, find_min!($($y),+));
    };
}

fn main() {
    // called macro
    say_hello!();

    foo();
    bar();

    // macro must be predefine than you can use it
    print_result!(1u32 + 1);

    test!(1i32 + 1 == 2i32; and 2i32 *2 == 4i32);
    test!(true; or false);
    println!("{}", test!(1; + 2));

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
