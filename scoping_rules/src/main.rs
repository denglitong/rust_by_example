// RAII（Resource Acquisition Is Initialization）资源获取即初始化，
// 或者叫作用域界定的资源管理（Scope-Bound Resource Management, SBRM）,
// RAII是一种C++编程技术，它将必须使用前请求的资源的生命周期与一个对象的生存期绑定，
// 保证资源可用于任何会访问该对象的函数，同时保证所有资源在其控制对象的生存周期结束时，以获取顺序的逆序释放，
// 这有效的利用了语言特性来消除内存泄露并保证异常安全（对象生存期、退出作用域、初始化顺序以及栈回溯）

// Box<T> owns memory in the heap

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    } // `_box3` out of scope here, and will be destroyed and memory gets freed

    for _ in 0u32..1_000 {
        create_box(); // no need to manually free memory
    }

    // `_box2` will get out of scope here, and will be destroyed and memory gets freed

    let _x = ToDrop;
    println!("Made a ToDrop!");

    let x = 5u32;
    // integer primitive type impl Copy trait, so no resources are moved
    let y = x;
    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    // resources moved, the pointer address of `a` is copied(not the data) into `b`,
    // both are now pointers in the stack to the same heap allocated data, but `b` now owns it,
    // and `a` can no logger access the data.
    let b = a;
    // println!("a contains: {}", a); // value borrow here after move

    // this function takes ownership of the heap allocated memory from `b`
    destroy_box(b);
    // println!("b contains: {}", b); // value borrow here after move

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    //*immutable_box = 4;

    // move the box, changing the ownership and mutability
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box contains {}", mutable_box);

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // cannot move outf of `box_i32` because it's borrowed
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);

    let immutable_book = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 2019,
    };

    // create a mutable copy of `immutable_book`
    let mut mutable_book = immutable_book;

    borrow_book(&immutable_book);
    // immutably borrow a mutable object
    borrow_book(&mutable_book);

    // borrow a mutable object as mutable
    // mutable reference generate from &mut mutable_var
    new_edition(&mut mutable_book);

    // new_edition(&mut immutable_book);

    // mutable borrow is mutex in current scope for any other mutable and immutable bindings
    let mut _mutable_integer = 7i32;
    {
        // borrow of `_mutable_integer` occurs here
        let large_integer = &_mutable_integer;
        // Error, assignment to borrowed `_mutable_integer` occurs here
        // _mutable_integer = 50;
        // in one scope, can not use both mutable reference and immutable reference
        println!("Immutably borrowed {}", large_integer);
    } // large_integer goes out of scope

    // ok, not both mutable and immutable reference occurs the same time
    _mutable_integer = 3;

    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        let mutable_borrow = &mut point;
        // when commented, ok; open comment, error for immutable and mutable borrow same scope
        // this is non-lexical lifetimes,
        // https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html
        // println!("{}", borrowed_point.x);
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        let y = &point.y;
        println!("Point Z coordinate is {}", point.z);

        let borrowed_point = &point;
        println!(
            "Point now has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y, borrowed_point.z
        );
    }

    let borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
} // `c` out of scopes and is destroyed and memory gets freed

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn create_box() {
    // allocate an integer on the heap
    let _box = Box::new(3i32);
} // `_box` is destroyed here, and memory gets freed
