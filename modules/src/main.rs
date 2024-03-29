mod my_mod_file;

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // can not see it...
    // crate::my_mod::public_function_in_my_mod();
    let open_box = my::OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    // public struct with private fields cannot be constructed using field names.
    // let closed_box = my::CloseBox {
    //     contents: "classified information",
    // };

    // however, structs with private fields can be created using public new fn
    let _closed_box = my::CloseBox::new("classified information");

    indirect_access();
    other_function();

    another_function();
    println!("Entering block");
    {
        // this is equivalent to `use deeply::nested::function as function`.
        // this `function` will shadow outer one.
        use crate::deeply::nested::function;
        function();

        println!("Leaving block");
    }
    function();

    my::indirect_call();

    my_mod_file::function();
    function();
    my_mod_file::indirect_access();
    my_mod_file::nested::function();
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

use deeply::nested::function as another_function;
use my_mod::function as other_function;
use my_mod::indirect_access;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents }
        }
    }

    fn function() {
        println!("called `my::function()`")
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
        self::function();
        function();

        self::cool::function();
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("call `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function`");
        }

        // functions declared using `pub(in path)` syntax are only visible within the given path.
        // `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested`");
        }

        // functions declared using `pub(super)` syntax are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`")
}
