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

#![allow(dead_code)]
use std::error::Error;
use std::f32::consts::E;
use std::fmt;
use std::fmt::Formatter;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{env, fs, io, thread, time};

static NTHREADS: u32 = 3;

static LOREM_IP_SUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

// FFI(Foreign Function Interface)
// Rust provides a FFI to C libraries. Foreign function must be declared inside an extern block
// annotated with a #[link] attribute containing the name of the foreign library

// minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Copy, Clone)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

// this extern block links to the libm library
#[link(name = "m")]
extern "C" {
    // this is a foreign function that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

// since calling foreign functions is considered unsafe, it's common to write safe wrappers around them.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn show_ffi() {
    let z = Complex { re: -1., im: 0. };
    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };
    println!("the square root of {:?} is {:?}", &z, z_sqrt);

    // calling safe API wrapped around unsafe function
    println!("cos({:?}) = {:?}", &z, cos(z));
}

// This is the `main` thread
fn main() {
    // show_threads_simple();
    // show_threads_map_reduce();
    // show_channels();
    // show_path();
    // show_file_open("hello.txt");
    // show_file_create();
    // show_file_read_lines();
    // show_child_process();
    // show_pipes();
    // show_process_wait();
    // show_fs_operation();
    // show_program_arguments();
    // show_arguments_parsing();
    show_ffi();
}

fn show_arguments_parsing() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        }
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer."),
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                }
            };

            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        _ => {
            help();
        }
    }
}

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
    );
}

fn show_program_arguments() {
    // type annotation is need
    let args: Vec<String> = env::args().collect();

    println!("My path is {}.", args[0]);

    println!("i got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}

// Filesystem operations, std::Fs
fn show_fs_operation() {
    println!("`mkdir a`");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // recursively create dir, returns `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // not found std::io::unix::fs; crate
    //    println!("`ln -s ../b.txt a/c/b.txt`");
    //    if cfg!(target_family = "unix") {
    //        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
    //            println!("! {:?}", why.kind());
    //        });
    //    }

    println!("`ls a`");
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

// a simple implementation of `# cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(why) => Err(why),
    }
}

// a simple implementation of `# echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(&path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn show_process_wait() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    println!("process beging");
    let _result = child.wait().unwrap();
    println!("reached end of process");
}

// Pipes: The std::Child struct represents a running child process, and exposes the stdin, stdout
// and stderr handlers for interaction with the underlying process via pipes.
static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
// echo "the quick brown fox jumped over the lazy dog\n" | wc
fn show_pipes() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance must have one,
    // we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => print!("wc responded with:\n{}", s), //        0       9      44
    }
}

// The process::Output struct represents the output of a finished child process,
// and the process::Command struct is a process builder.
fn show_child_process() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }
}

fn show_file_read_lines() {
    if let Ok(lines) = read_lines("/etc/passwd") {
        // consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(row) = line {
                println!("{}", row);
            }
        }
    }

    let file = File::open("/Users/leon/.bashrc");
    let lines = io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(row) = line {
            println!("{}", row);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn show_file_create() {
    // The `create` static method opens a file in write-only mode.
    // If the file already existed, the old content is destroyed, otherwise a new file is created
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    //    let file = OpenOptions::new()
    //        .read(true)
    //        .write(true)
    //        .create(true)
    //        .open(display);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IP_SUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    show_file_open(display.to_string().as_str());
}

fn show_file_open(file_name: &str) {
    let path = Path::new(file_name);
    let display = path.display();

    // The `open` static method can be used to open a file in read-only mode.
    let mut file = match File::open(&path) {
        // the description method of io::Error returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file goes closed (Drop trait)
}

// two flavors of Path: posix::Path, for UNIX-like systems, and windows::Path, for Windows
// Note that a Path is not internally represented as an UTF-8 string, but instead is stored as a
// vector of bytes(Vec<u8>), Therefore, coverting a Path to &str may fail(an Option is returned)
fn show_path() {
    let path = Path::new(".");
    // the `display` method returns a `Show`able structure
    let display = path.display();
    println!("{:?}", display);

    let meta = path.metadata();
    println!("metadata: {:?}", meta);
    match meta {
        Ok(m) => {
            println!("{:?}", m);
            println!("file_type: {:?}", m.file_type());
            println!("is_dir: {:?}", m.is_dir());
            println!("is_file: {:?}", m.is_file());
            println!("permissions: {:?}", m.permissions());
            println!("modified: {:?}", m.modified());
            println!("accessed: {:?}", m.accessed());
            println!("created: {:?}", m.created());
        }
        Err(r) => panic!("path {:?} is not valid", path),
    }

    // `join` merges a path with a byte container using the OS specific separator, and returns the new path
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

fn show_channels() {
    println!("======= show_channels =========");

    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel(); // mpsc: multiple producer single consumer
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // the sender endpoint can be copied
        let thread_tx = tx.clone();

        // each thread will send its id via the channel
        let child = thread::spawn(move || {
            // the thread takes ownership over `thread_tx`
            // each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // sending is a non-blocking operation, the thread will continue immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // collect all the messages
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // the `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // show the order in which the messages were sent
    println!("{:?}", ids);
}

fn show_threads_map_reduce() {
    println!("======= show_threads_map_reduce =========");

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

fn show_threads_simple() {
    println!("======= show_threads_simple =========");

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
