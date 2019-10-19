// Rust的错误处理
// 1.显示的 panic 主要用于测试，以及处理不可恢复的错误
// 2.Option 类型是为了值是可选的、或者缺少值并不是错误的情况准备的
// 3.当错误有可能发生且应当由调用者处理是，使用 Result

fn main() {
    give_princess("teddy bear");
    // give_princess("snake");

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess_2(bird);
    give_princess_2(nothing);
}

fn give_princess_2(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaaa!");
    }
    println!("I love {}s!!!!!", inside);
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAaaaaaa!!!");
    }
}
