// Types

type NanoSecond = u64;
type Inch = u64;

//#[allow(non_camel_case_types)]
type u64_t = u64; // 类型名称命名需要使用双驼峰格式
type U64T = u64;

// 类型别名的用途主要用来节省样板代码

fn main() {
    let decimal = 64.4321_f32; // also 64.4321f32
    println!("{}", decimal);

    // let integer: u8 = decimal; // error, no implicit conversion
    let integer = decimal as u8; // explicit conversion
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value fits into the new type
    let decimal = 256;
    let integer: u8 = decimal as u8;
    println!("Casting: {}_i32 -> {}_u8", decimal, integer); // 256_i32 -> 0_u8

    // 1000 already fits in a u16
    let decimal = 1000;
    println!("1000 as a u16 is: {}", decimal as u16);
    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is: {}", decimal as u8);
    // -1 + 256 = 255
    println!(" -1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);

    // when casting to a signed type, the(bitwise) result is the same as first casting to the corresponding unsigned type.
    // if the most significant bit of that value is 1, then the value is negative.

    println!("128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is: {}", 128_u8 as i8); // 128 -> 127 + 1 -> -128

    println!("1000 as a u8 is: {}", 1000_u32 as u8);
    // 232 - 127 - 1 -128 = -24
    println!("232 as a i8 is: {}", 232_u32 as i8);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // std::mem::size_of_val()
    // crate::module::fn()

    let elem = 5u8;
    // a this point the compiler doesn't know the exact type o `vec`,
    // it just knows that it's a vector of something (`Vec<_>`)
    let mut vec = Vec::new();
    println!("{:?}", vec);

    vec.push(elem);
    // now the compiler knows that `vec` is a vector of `u8`
    println!("{:?}", vec);

    // NanoSecond = u64 = Inch = u64_t
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
