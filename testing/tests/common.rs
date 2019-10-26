// define a crate in tests:
// 1.tests/mod_name.rs
// (Each Rust source file in tests directory is compiled as a separate create)
// or
// 2.tests/mod_name/mod.rs

pub fn setup() {
    println!("common setup");
}
