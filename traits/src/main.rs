fn main() {
    // you can use Trait static method and type annotation to call implement method
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

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
