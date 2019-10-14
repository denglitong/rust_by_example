mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my_mod_file::function()`");
}

fn private_function() {
    println!("called `my_mod_file::private_function()`");
}

pub fn indirect_access() {
    print!("called `my_mod_file::indirect_access()`, that\n> ");
    private_function();
}
