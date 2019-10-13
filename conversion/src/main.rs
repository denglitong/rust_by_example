#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
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
}
