// all values in Rust are stack allocated by default.
// values can be boxed(allocated on the heap) by creating a Box<T>.
// a Box<T> is smart pointer to a heap allocated value of type t.
// boxed values can be dereferenced using * operator
#![allow(dead_code)]

use std::collections::HashMap;
use std::mem;
use std::str;
use std::string;

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

    // a reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown for jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_ascii_whitespace().rev() {
        println!("> {}", word);
    }

    // copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("{:?}", chars);

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("{}", string);

    // the trimmed string is a slice to the original string, hence no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocated a string
    let alice = String::from("I like dogs");
    // allocate new memory and store the modidifed string there
    let bob = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let mut s = "hello world";
    println!("{:p}", s);
    s = "hello";
    // address changed, that is, &str is stored in binary allocation and can not change the content,
    // you can only change the pointer value to point to another address.
    println!("{:p}", s);

    // 转义 you can use escapes to write bytes by their hexadecimal values
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // or Unicode code points
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D is called {})",
        unicode_codepoint, character_name
    );

    // 多行
    let long_string = "String literals \
                       can span multiple lines.\
                       The linebreak and indentation here =>\
                       <- can be escaped too!";
    println!("{}", long_string);

    // 原始字符串
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 使用##对来保存字符串里面的双引号
    let quotes = r#"And the I said: "There is no escape!""#;
    println!("{}", quotes);

    // if you need "# in your string, just use more #s in the delimiter,
    // there is no limit for the number of #s you can use
    let longer_delimiter = r###"A tring with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // note that this is not actually a `&str`
    let byte_string: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", byte_string);

    // byte strings can have byte escapes,
    let escaped = b"\x52\x75\x74\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);
    // but unicode string can not have byte escapes
    // let escaped = b"\u{211D} is not allowed";
    // println!("Some escaped bytes: {:?}", escaped);

    let raw_byte_string = br"\u{211D} is not escaped here";
    println!("{:?}", raw_byte_string);

    // converting a byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_byte_string) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"also "fancier" formatting, \
                    like with normal raw strings"#;
    // std::array::LengthAtMost32 is not implemented for [u8; 77]
    // println!("{:?}", _quotes);

    // byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\xa8\xb1\x82";
    // println!("{}", shift_jis); // can not print non-utf8 chars
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successfule: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }

    try_division(4, 2);
    try_division(1, 0);

    let optional_float = Some(0f32);
    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );

    //println!("{:?} unwraps to {:?}", None, None.unwrap())

    // println!("{}", op(1.0, 10.0));
    println!("{}", op(10.0, 1.0));

    println!("{:?}", checked::op(1.0, 10.0));
    println!("{:?}", checked::op(10.0, 1.0));

    let _x = Box::new(0i32);
    // division(3, 0);
    // println!("This point won't be reached!");

    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    let ins = contacts.insert("Newkey", "164-6743");
    // if the inserted value is new, then return `None`
    println!("{:?}", ins);
    let ins = contacts.insert("Daniel", "164-6743");
    // if the inserted value existed, then return `Some(value)`
    println!("{:?}", ins);

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number"),
    }

    contacts.remove(&"Ashely");

    // HashMap::iter() returns an iterator that yields (&'a key, &'a value) pair in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("- Calling {}: {}", contact, call(number));
    }

    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "password123");
    try_logon(&accounts, "j.everyman", "pasaword123");
}

// any type that implements the Eq and Hash traits can be a key in HashMap, this includes:
// bool
// int, uint
// String and &str (pro tip: you can have a HashMap key by String and call .get() with an &str)
// 封装类型的 Eq 和 Hash 取决于其基础类型的 Eq, Hash
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("username: {}", username);
    println!("password: {}", password);
    println!("Attempting logon...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}, Email: {}", account_info.name, account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "we're sorry, the call cannot be completed as dialed.\
             Please hang up and try again"
        }
        "645-7689" => {
            "hello, this is Mr. Awesome's Pizza. My name is Fred.\
             What can I get from you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

// panic! macro can be used to generate a panic and start unwinding its stack.
// while unwinding, the runtime will take care of freeing all the resources owned by the thread
// by calling the destructor of all its objects.
// if you dealing with programs with only one thread, panic! will cause the program to exit.
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero")
    } else {
        dividend / divisor
    }
}

mod checked {
    #[derive(Debug)]
    pub enum MatchError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MatchResult = Result<f64, MatchError>;

    pub fn div(x: f64, y: f64) -> MatchResult {
        if y == 0.0 {
            Err(MatchError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MatchResult {
        if x < 0.0 {
            Err(MatchError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MatchResult {
        if x <= 0.0 {
            Err(MatchError::NonPositiveLogarithm)
        } else {
            // 自然对数 logeX
            Ok(x.ln())
        }
    }

    // using ?
    pub fn op(x: f64, y: f64) -> MatchResult {
        let quotient = div(x, y)?;
        let ln = ln(quotient)?;
        sqrt(ln)
    }
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // failure is represented as the `None` variant
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}

// Strings: // there are two types of strings in Rust: string and &str
// A String is stored as a vector of bytes(Vec<u8>), but guaranteed to always be a valid UTF-8 sequence,
// String is heap allocated, growable and not null terminated
//
// &str is a slice(&[u8]) that always points to valid UTF-8-sequence,
// and can be used to view into a String, just like &[T] is a view into Vec<T>

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
