fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // blocks are expressions too, so they can be used as values in assignments
    // the last repression in the block will be assigned to the place expression
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // this expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // the semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
