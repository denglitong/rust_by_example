#![allow(dead_code)]
// Rust的错误处理
// 1.显示的 panic 主要用于测试，以及处理不可恢复的错误
// 2.Option 类型是为了值是可选的、或者缺少值并不是错误的情况准备的
// 3.当错误有可能发生且应当由调用者处理是，使用 Result

use core::fmt;
use std::error;
use std::fmt::{Error, Formatter};
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    give_princess("teddy bear");
    // give_princess("snake");

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing = None;

    give_princess_2(bird);
    // give_princess_2(nothing);

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    eat_v2(cordon_bleu, Day::Monday);
    eat_v2(steak, Day::Tuesday);
    eat_v2(sushi, Day::Wednesday);

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // panic
    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);

    let tt = multiply_v2("t", "2");
    print(tt);

    let twenty = multiply_v3("10", "2");
    print(twenty);

    let tt = multiply_v3("t", "2");
    print(tt);

    print_v2(multiply_v4("10", "2"));
    print_v2(multiply_v4("t", "2"));

    print_v2(multiply_v5("10", "2"));
    print_v2(multiply_v5("t", "2"));

    print_v2(multiply_v6("10", "2"));
    print_v2(multiply_v6("t", "2"));

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers.clone()));
    // println!("The first doubled is {}", double_first(empty));
    // println!("The first doubled is {}", double_first(strings));

    println!(
        "The first doubled is {:?}",
        double_first_v2(numbers.clone())
    );
    println!("The first doubled is {:?}", double_first_v2(empty.clone()));
    println!(
        "The first doubled is {:?}",
        double_first_v2(strings.clone())
    );

    println!(
        "The first doubled is {:?}",
        double_first_v3(numbers.clone())
    );
    println!("The first doubled is {:?}", double_first_v3(empty.clone()));
    println!(
        "The first doubled is {:?}",
        double_first_v3(strings.clone())
    );

    print_v3(double_first_v4(numbers.clone()));
    print_v3(double_first_v4(empty.clone()));
    print_v3(double_first_v4(strings.clone()));

    print_v4(double_first_v5(numbers.clone()));
    print_v4(double_first_v5(empty.clone()));
    print_v4(double_first_v5(strings.clone()));

    print_v4(double_first_v6(numbers.clone()));
    print_v4(double_first_v6(empty.clone()));
    print_v4(double_first_v6(strings.clone()));

    Ok(())
}

fn double_first_v6(vec: Vec<&str>) -> BoxResult<i32> {
    // here ? use From::from(e) to auto convert error type the the return one if it's convertible
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

type BoxResult<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn double_first_v5(vec: Vec<&str>) -> BoxResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // convert to box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // convert to box
                .map(|i| 2 * i)
        })
}

fn print_v4(result: BoxResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[derive(Debug, Clone)]
struct DoubleError;

type MyResult<T> = std::result::Result<T, DoubleError>;

impl std::fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn double_first_v4(vec: Vec<&str>) -> MyResult<i32> {
    vec.first()
        .ok_or(DoubleError) // change the error to our new type.
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError) // update to the new error type here also
                .map(|i| 2 * i)
        })
}

fn print_v3(result: MyResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// a couple of combinators come in handy to swap the Result and Option:
fn double_first_v3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    let opt = opt.map_or(Ok(None), |r| r.map(Some))?;

    Ok(opt)
}

fn double_first_v2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // generate error 1, Option::None.unwrap()
    2 * first.parse::<i32>().unwrap() // generate error 2, Result::unwrap() -> ParseIntError
}

// the old try!
//fn multiply_v7(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
//    // try! is deprecated in 1.39.0, use ? instead
//    let first_number = try!(first_number_str.parse::<i32>());
//    let second_number = try!(second_number_str.parse::<i32>());
//    Ok(first_number * second_number)
//}

fn multiply_v6(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn multiply_v5(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

// Result<T,E> alias
type AliasResult<T> = Result<T, ParseIntError>;

fn multiply_v4(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print_v2(result: AliasResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn multiply_v3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    // and_then() calls its function input with the wrapped value and returns the result,
    // if the option is None, the it returns None instead
    have_recipe(food).and_then(have_ingredients)
}

fn eat_v2(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! Oh {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(food: Option<Peeled>) -> Option<Chopped> {
    match food {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(food: Option<Chopped>) -> Option<Cooked> {
    match food {
        Some(Chopped(food)) => Some(Cooked(food)),
        None => None,
    }
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

fn give_princess_2(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaaa!");
    }
    println!("I love {}s!!!!!", inside);
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAaaaaaa!!!");
    }
}
