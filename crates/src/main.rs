// link to `library`, import items under the `rary` module
extern crate rary;

// rustc main.rs --extern rary=library.rlib && ./main

fn main() {
    rary::public_function();

    rary::indirect_access();
}
