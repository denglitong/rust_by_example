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
}
