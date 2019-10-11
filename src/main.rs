fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    // positional argument
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    // special formatting, {:b} :binary
    println!("{} of {:b} people know binary, the other half doesn't",1,2);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3));

    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` print...", Structure(3));

    let pi = 3.141592;
    println!("{:.2}", pi);
    println!("{0:.2}", pi);
    println!("{1:.0$}", 2, pi);
    println!("{:.*}", 2, pi);
    println!("{number:.prec$}", prec = 2, number = pi);
}
