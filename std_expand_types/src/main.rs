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
use std::thread::JoinHandle;

static NTHREADS: u32 = 3;

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

    // The standard library provides great threading primitives out of the box.
    // These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.
    // The aliasing rules(one writable reference XOR many readable references) automatically prevent
    // you from manipulating state that is visible to other threads.
    // and synchronisation primitives like Mutex or Channel is for synchronisation situation

    // although we're passing references across thread boundaries, Rust understands that we're only
    // passing read-only references, and that thus no unsafety or data races can occur.
    // Because we're move-ing the data segments into the thread, and Rust will also ensure the data
    // is kept alive until the threads exit, so no dangling pointers occur.
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    // make a vector to hold the child-threads which we will spawn
    let mut children = vec![];

    // `Map` phase
    // split data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();
    println!("{:?}", chunked_data);

    let mut threads = 0u32;
    let mut final_result = 0u32;

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // process each data segment in s separate thread.
        // spawn() returns a handle to the new thread, which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that;
        // * take no arguments ('||')
        // * takes ownership of its captured variables ('move')
        // * returns an unsigned 32-bit integer
        // if remove the `move`, you will ge compile error:
        // closure borrows `data_segment`, but closure my outlive the current function
        // and `data_segment` is owned by the current function, so the closure need to own the data
        // or data be static lifetimes

        children.push(thread::spawn(move || -> u32 {
            // calculate the intermediate sum of this segment:
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);

            result
        }));

        threads += 1;
        if threads >= NTHREADS {
            let mut intermediate_sums = vec![];
            for child in children {
                intermediate_sums.push(child.join().unwrap());
            }

            // combine all intermediate sums into a single final sum.
            // we use the "turbofish" ::<> to provide sum() with a type hint.
            final_result += intermediate_sums.iter().sum::<u32>();

            println!("threads: {}, result: {}", threads, final_result);
            children = vec![];
            threads = 0;
        }
    }

    if threads > 0 {
        let mut intermediate_sums = vec![];
        for child in children {
            intermediate_sums.push(child.join().unwrap());
        }

        // combine all intermediate sums into a single final sum.
        // we use the "turbofish" ::<> to provide sum() with a type hint.
        final_result += intermediate_sums.iter().sum::<u32>();

        println!("threads: {}, result: {}", threads, final_result);
        children = vec![];
        threads = 0;
    }

    // let final_result = intermediate_sums.iter().sum(); // can not infer type
    println!("Final sum result: {}", final_result);
}
