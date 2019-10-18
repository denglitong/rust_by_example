use std::ops::Add;

fn main() {
    // you can use Trait static method and type annotation to call implement method
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let _one_second = Seconds(1);
    // println!("One second looks like: {:?}", _one_second); // no Debug trait impl for Seconds
    // let _this_is_true = (_one_second == _one_second); // no PartialEq trait impl for Seconds

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter", cmp);

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B")
        }
        println!("Just exited block B")
    }
    println!("Just exited block A");

    // var can be manually dropped using the `drop` funciton
    drop(_a);
    println!("end if the main function");
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

// in Rust many of operators can be overload via traits, because they are method implement in trait.
// in front-end we see operator because the operator is the syntax sugar
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

impl std::ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> Self::Output {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl std::ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, rhs: Foo) -> Self::Output {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

// Eq, PartialEq, Ord, PartialOrd: comparision traits
// Clone: to create T from &T via a copy
// Copy: to give a type 'copy semantics' instead of 'move semantics'
// Hash: to compute a hash from &T
// Default: to create an empty instance of a data type
// Debug: to format a value using the {:?} formatter
#[derive(PartialOrd, PartialEq)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // static method signature: `Self` refers to the implement type.
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn notice(&self) -> &'static str;

    // trait can provide default method definitions
    fn talk(&self) {
        println!("{} syas {}", self.name(), self.notice());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name);
        } else {
            println!("{} get a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: &name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn notice(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaaah?"
        } else {
            "baaaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} paues briefly... {}", self.name, self.notice())
    }
}
