use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct DisplayStructure(i32);

impl fmt::Display for DisplayStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    // positional argument
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // special formatting, {:b} :binary
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3));
    println!("This struct `{:?}` print...", Structure(3));

    let pi = 3.141592;
    println!("{:.2}", pi);
    println!("{0:.2}", pi);
    println!("{1:.0$}", 2, pi);
    println!("{:.*}", 2, pi);
    println!("{number:.prec$}", prec = 2, number = pi);

    struct UnPrintable(i32);
    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!("{:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
    // pretty print，格式化输出保持缩进
    println!("{:#?}", peter);

    let point = Point { x: 1, y: 2 };
    println!("{}", point);

    let display_structure = DisplayStructure(3);
    println!("{}", display_structure);

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big Display range is {big} and small is {small}",
        small = small_range,
        big = big_range
    );
    println!(
        "The big Debug range is {big:?} and small is {small:?}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Debug: {key:?}", key = point);
    // println!("What does Point2d look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
