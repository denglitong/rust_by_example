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

fn main() {
    // called macro
    say_hello!();
}

// macro is useful:
// 1.DRY: don't repeat yourself
// 2.Domain-specific languages, macros allow you to define special syntax for a specific purpose
// 3.variadic interfaces
