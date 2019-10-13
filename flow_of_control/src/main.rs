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
}
