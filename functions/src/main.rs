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
