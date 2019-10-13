// custom types:
// struct: define a structure
// enum: define an enumeration
// const、static define constants

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
}
