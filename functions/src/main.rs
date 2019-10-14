fn main() {
    fizzbuzz(100);

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(2.0, 2.0);
    println!("square: p1: {:?}, p2: {:?}", square.p1, square.p2);
    println!("square area: {}", square.area());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // pair.destroy(); // has already moved and can not use again

    fn function(i: i32) -> i32 {
        i + 1
    }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i)); // infer type for closure

    let one = || 1;
    println!("closure returning one: {}", one());

    // closures can capture variables:
    // by reference: &T
    // by mutable reference: &mut T
    // by value: T
    let color = "green";
    let print = || println!("`color: {}`", color);
    print(); // call closures using the borrow
    print();

    let mut count = 1;
    // a `mut` is needed for closure literal because a `&mut` is stored inside
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc(); // call the closure using the mut
    inc();
    let _reborrow = &mut count;

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        // std::mem::drop requires `T` so this must take by value
        // a non-copy must move and so `movable` immediately moves into the closure
        std::mem::drop(movable);
    };
    consume();
    // consume();

    // `Vec` has non-copy semantics
    let haystack = vec![1, 2, 3];
    // using move before vertical pipes forces closure to take ownership of captured variables
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4)); // moved closure can reuse, by variables moved in cannot reuse
                                  // println!("{}", haystack.len());

    // closure as parameters need to be type annotate, and closure's complete type use:
    // Fn: the closure captures by reference &T, immutable borrow
    // FnMut: the closure captures by mutable reference &mut T, mutable borrow
    // FnOnce: the closure captures by value T, move
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}.", greeting);
        // mutation forces `farewell` to be captured by mutable reference, now requires `FnMut`
        farewell.push_str("!!!");
        println!("Now I can sleep. zzzzz");

        // manually calling drop forces `farewell` to be captured by value, now requires `FnOnce`
        std::mem::drop(farewell);
    };
    // 当闭包定义的时候，编译器会隐式创建一个匿名的struct将上下文变量保存在里面，然后对闭包实现Fn/FnMut/FnOnce
    // 其中的一个trait, 此时闭包的type还是未知的，知道闭包被调用的时候type信息才会被保存

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    let double_str = |str| {
        let mut s: String = String::from(str);
        s.push_str(String::from(str).as_str());
        s
    };
    let a = "hello";
    println!("double str: {}", double_str(a));
    println!("a: {}", a);

    let x = String::from("7");
    let print = || {
        // need borrow here, can not be value moved use!
        // the type of closure is dependent on how you capture scope variables
        let a = &x;
        println!("{}", a);
    };
    apply_mutable_borrow(print);
    println!("x: {}", x);

    let closure = || println!("I'm a closure");
    call_me(closure);
    call_me(function_print);

    let closure_add = |a, b| a + b;
    println!("1 + 2 = {:?}", call_take_params(closure_add, 1, 2));
    println!("1 + 2 = {:?}", call_take_params(function_add, 1, 2));

    let fn_plain = create_fn();
    fn_plain();
    let mut fn_mut = create_fnmut();
    fn_mut();

    fn_mut = Box::new(|| {
        let text = "modify fn_mut".to_owned();
        println!("{}", text);
    });
    fn_mut();

    let (a, b) = (1, 2);
    let fn_add = create_fn_add();
    println!("1 + 2 = {}", fn_add(a, b));
    println!("a: {}, b: {}", a, b);

    let (a, b) = ("hello, ".to_owned(), "world".to_owned());
    let fn_append = create_fn_append();
    println!("a + b = {}", fn_append(a, b));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // Iterator::any return predicate if any item satisfied closure
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.iter().any(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.iter().any(|&x| x == 2));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // Iterator::found return the first element satisfied the closure
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
    // println!("{:?}", vec2); // into_iter will move the vec

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("2 in array2: {:?}", array2.iter().find(|&&x| x == 2));

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // imperative approach
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // all natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // that are odd
        .fold(0, |acc, n_squared| acc + n_squared); // sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

// higher order functions
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// closure as output parameters: Rust currently only supports returning concrete types(non-generic)
// anonymous closure by definition is unknown types so returning a closure is only possible by making it concrete
// this can be done via boxing.
// Fn: normal
// FnMut: normal
// FnOnce: there are some unusual things here, so the FnBox type is currently needed, and is unstable
// this is expected to change in future

// Beyond this, the move keyword must be used for returning closure, because any captures by reference
// would by dropped as soon as the function existed.
fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn create_fn_add() -> Box<Fn(u32, u32) -> u32> {
    Box::new(move |a, b| a + b)
}

fn create_fn_append() -> Box<FnOnce(String, String) -> String> {
    Box::new(move |a, b| {
        let mut s = a;
        s.push_str(b.as_str());
        s
    })
}

fn function_add(a: u32, b: u32) -> u32 {
    a + b
}

// define function take closure, then function satisfy trait bound of that closure can be passed as parameter
// Fn, FnMut, FnOnce trait dictate how a closure captures variables from the enclosing scope
fn call_me<F: Fn()>(f: F) {
    f();
}

fn call_take_params<F>(f: F, a: u32, b: u32) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    f(a, b)
}

fn function_print() {
    println!("I'm a function!");
}

// closures in Rust, also called lambda expression, are functions that can capture the enclosing environment
// FnOnce: closure capture value T
// 仅仅使用<F>的泛型类型来声明闭包的类型还是不够的，还需要使用Fn,FnMut,FnOnce来声明，这时类型信息就够了
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// Fn: mutable borrow
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn apply_mutable_borrow<F>(f: F)
where
    F: Fn(),
{
    f();
}

// methods are functions attached to objects
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // need a mutable self
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // get out of scope and get freed
    }
}

// functions that "don't' return a value, actually return the unit type `()`
// (in Rust everything is type/repression?)
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

// when a fn returns `()`, then return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
