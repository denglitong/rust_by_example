// documentation: rustdoc
// testing: create test suites for libraries to give confidence that your library does exactly
// what it's supposed to
// benchmarking: create benchmarks for functionality to be confident that they run quickly
#![crate_name = "meta"]
/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// use meta::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let john = Person::new("John");
    john.hello();
}
