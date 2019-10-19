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

use std::fs::create_dir_all;
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a: ident, $b: ident, $func: ident, $op: tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?} dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func: ident, $bound: ident, $op: tt, $method: ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x: expr, $y: expr, $z: expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

macro_rules! calculate {
    // {{ }} 这里的双大括号是macro语法的一部分，使得可以使用 {} () [] 给macro传参
    (eval $e: expr) => {{
        {
            let val: usize = $e; // force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    (eval $e: expr, $(eval $es: expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }}
}

fn main() {
    // call macro
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

    calculate! {
        eval 1 + 2 // here `eval` is _not_ a Rust keyword!
    }
    calculate!(eval 1 + 2);
    calculate![eval 1 + 2];

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    calculate! (
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    )
}
