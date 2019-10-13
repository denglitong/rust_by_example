// custom types:
// struct: define a structure
// enum: define an enumeration
// const、static define constants

// an attribute to hide warnings for unused code.
#![allow(dead_code)]

use crate::List::Cons;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// struct can be reused as fields of another struct
//#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = *rect;
    (x1 - x2).abs() * (y1 - y2).abs()
}

fn square(point: &Point, width: f32) -> Rectangle {
    Rectangle {
        p1: Point {
            x: point.x,
            y: point.y,
        },
        p2: Point {
            x: point.x + width.abs(),
            y: point.y + width.abs(),
        },
    }
}

enum WebEvent {
    // an `enum` may either by `unit-like`
    PageLoad,
    PageUnLoad,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum with implicit discriminator(starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: a node that signifies the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    // consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // return the length of the list
    fn len(&self) -> u32 {
        match *self {
            // `self` has to be matched, because the behavior of this method depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T`
            // is preferred over a match on a reference `&T`
            List::Nil => 0,
            List::Cons(_, ref tail) => 1 + tail.len(),
        }
    }

    // return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Nil => format!("Nil"),
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
        }
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    println!("my_x: {}, my_y: {}", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point {
            x: my_x * 2.0,
            y: my_y * 2.0,
        },
        p2: point,
    };
    println!("rectangle: {:?}", _rectangle);
    println!("rectangle area: {:.2}", rect_area(&_rectangle));
    println!("square: {:?}", square(&_rectangle.p1, 3.0));
    println!(
        "square area: {:.2}",
        rect_area(&square(&_rectangle.p1, 3.0))
    );

    // instantiate a unit struct
    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my_test".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // `enum` can be cast as integer
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
