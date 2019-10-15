// default bin is main.rs, multiple bin can be placed in bin/*.rs
// cargo [build|run] --bin [my_other_bin|${project_name}]
// which cargo run --bin ${project_name} equals default main.rs

fn main() {
    println!("hello world in my_other_bin.");
}
