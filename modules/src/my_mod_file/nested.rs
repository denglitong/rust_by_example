pub fn function() {
    println!("called `my_mod_file::nested::function()`")
}

#[allow(dead_code)]
pub fn public_function() {
    println!("called `my_mod_file::nested::public_function()`");
}
