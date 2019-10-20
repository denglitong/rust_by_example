// all values in Rust are stack allocated by default.
// values can be boxed(allocated on the heap) by creating a Box<T>.
// a Box<T> is smart pointer to a heap allocated value of type t.
// boxed values can be dereferenced using * operator

use std::mem;

fn main() {
    // stack allocated variables
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 3.0 },
    };

    // heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // the output of function can be boxed
    let boxed_point = Box::new(origin());

    // double indirection
    let box_in_a_box = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size, 8 bytes
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );

    let collected_iterator: Vec<i32> = (0..10).collect();
    println!(
        "Collected(0..10) into {:?}, size: {} bytes",
        collected_iterator,
        mem::size_of_val(&collected_iterator), // 24: pointer size 8, usize length 8, capacity 8
    );
    // usize bytes: 8, the pointer-sized unsigned integer type
    println!("usize bytes: {}", mem::size_of::<usize>());
    // isize bytes: 8, the pointer-sized signed integer type
    println!("isize bytes: {}", mem::size_of::<isize>());

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(4);
    println!("Vector: {:?}", xs);

    // error, not mut variable
    // collected_iterator.push(0);

    println!("Vector length: {}", xs.len());
    println!("Second element: {}, {:?}", xs[1], xs.get(1));
    // vec `pop` removes the last element from the vector and return it
    println!("Pop last element: {:?}", xs.pop());

    // out of bounds error
    // println!("Fourth element: {}", xs[3]);

    println!("Contents of xs: ");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}

// Vectors are -re-sizable arrays. like slices, their size is not known at compile time, but then
// can grow or shrink at any time. A vector is representing using 3 parameters:
// pointer to the data
// length
// capacity

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(origin())
}
