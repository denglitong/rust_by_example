// primitive types:
// Scalar types: 标量类型
// signed integers: i8, i16, i32, i64, i128, isize (pointer size, 1 byte)
// unsigned integers: u8, u16, u32, u64, u128, usize (pointer size, 1 byte)
// floating point: f32, f64
// char: Unicode scalar values like 'a', 'α', and '∞' (4 bytes)
// bool: true false
// (): unit type

// Compound Types:
// arrays: [1, 2, 3]
// tuple: (1, true)

// Threads
// Channels
// File I/O
// ...

use std::thread;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    // make a vector to hold the children which are spawned
    let mut children = vec![];

    for i in 0..NTHREADS {
        // spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // wait for the thread to finish. returns a result
        let _ = child.join();
    }
}
