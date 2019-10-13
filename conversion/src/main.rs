use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

struct Circle {
    radius: i32,
}

// implement the fmt::Display trait for type which will automagically provides ToString
// and also allows printing the type on print!
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("{}", my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // if you have implemented the From trait for you type,
    // you get the Into trait implementation for free
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let circle = Circle { radius: 6 };
    println!("{}", circle);
    println!("{}", circle.to_string());

    // 标准库中的数字类型实现了 FromStr trait，可以从 String 调用 parse 函数转化到对应的数字类型
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);
}
