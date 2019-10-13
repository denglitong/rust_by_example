fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // 内存loop可以直接跳出外层loop，前提是使用loop的'label: loop{}语法
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // 使用 break expression; 可以将值作为loop的返回结果
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            // println!("{}", n);
        }
        n += 1;
    }

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            // println!("{}", n);
        }
    }

    for n in 1..=3 {
        println!("{}", n);
    }

    // for and iterators
    // iter - 不可变借用
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a restacean amon us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - 移动所有权
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a restacean amon us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("{:?}", names); // error: value borrowed here after move

    // iter_mut - mut borrow
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a restacean amon us!",
            _ => "Hello",
        }
    }
    println!("{:?}", names);

    let number = 13;
    match number {
        // single value
        1 => println!("One"),
        // several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // an inclusive range
        13...19 => println!("A teen"),
        // the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    let binary = if boolean { 1 } else { 0 };
    println!("{} -> {}", boolean, binary);

    // match destructure tuples
    let pair = (0, -2);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}", r, g, b),
        Color::HSV(r, s, v) => println!("Hue: {}, saturation: {}, and value: {}", r, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => {
            println!("Cyan: {}, magenta: {}, yellow: {}, key: {}", c, m, y, k);
        }
    }

    // pointers/ref
    // dereferencing uses *
    // destructuring uses &, ref and ref mut
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructing: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    // Rust provides `ref` for exactly modifies the assignment so that a reference is created for the element,
    // this reference is assigned
    let ref _is_a_reference = 3;
    let a = _is_a_reference; // a is &i32

    // references can be retrieved via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => {
            let v = r; // v is &i32
            println!("Got a reference to a value: {:?}", v);
        }
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value: {:?}`", m); // 16
        }
    }
    println!("mut_value: {}", mut_value); // 16
}

// the compiler provides a dead_code lint that will warn about unused functions,
// the allow(dead_code) will disable the lint
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
