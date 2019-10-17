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
