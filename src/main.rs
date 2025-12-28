// =============================================================================
// RUST OOP CONCEPTS DEMONSTRATION
// =============================================================================
// Rust doesn't have traditional class-based inheritance. Instead it uses:
// - Traits for ABSTRACTION and POLYMORPHISM
// - Visibility modifiers for ENCAPSULATION
// - Composition + Default trait methods for "INHERITANCE"-like behavior
// =============================================================================

// =============================================================================
// 1. ABSTRACTION - Define behavior without implementation details
// =============================================================================
// Traits define WHAT an object can do, not HOW it does it

trait Animal {
    // Abstract method - each type MUST implement
    fn speak(&self) -> String;
    fn name(&self) -> &str;
    
    // Default implementation - "inherited" by all implementors
    fn describe(&self) -> String {
        format!("{} says: {}", self.name(), self.speak())
    }
}

// Secondary trait for additional behavior
trait Swimmer {
    fn swim(&self) -> String {
        String::from("Swimming...")
    }
}

// =============================================================================
// 2. ENCAPSULATION - Hide internal state, expose controlled interface
// =============================================================================

pub struct Dog {
    name: String,           // Private - cannot access directly from outside
    age: u8,                // Private
    breed: String,          // Private
}

impl Dog {
    // Public constructor - controlled way to create instances
    pub fn new(name: &str, age: u8, breed: &str) -> Self {
        Self {
            name: name.to_string(),
            age,
            breed: breed.to_string(),
        }
    }
    
    // Public getter - controlled read access
    pub fn get_age(&self) -> u8 {
        self.age
    }
    
    // Public setter with validation - controlled write access
    pub fn set_age(&mut self, age: u8) {
        if age <= 25 {  // Dogs rarely live beyond 25
            self.age = age;
        }
    }
    
    // Private helper method - internal use only
    fn format_breed(&self) -> String {
        self.breed.to_uppercase()
    }
}

// Implement the Animal trait for Dog
impl Animal for Dog {
    fn speak(&self) -> String {
        format!("Woof! I'm a {}", self.format_breed())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// =============================================================================
// 3. POLYMORPHISM - Same interface, different behaviors
// =============================================================================

pub struct Cat {
    name: String,
    indoor: bool,
}

impl Cat {
    pub fn new(name: &str, indoor: bool) -> Self {
        Self {
            name: name.to_string(),
            indoor,
        }
    }
}

// Same trait, different implementation
impl Animal for Cat {
    fn speak(&self) -> String {
        if self.indoor {
            String::from("Meow~ (comfortable purr)")
        } else {
            String::from("MEOW! (street cat attitude)")
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Duck {
    name: String,
}

impl Duck {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Animal for Duck {
    fn speak(&self) -> String {
        String::from("Quack quack!")
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Duck can also swim - multiple trait implementation
impl Swimmer for Duck {
    fn swim(&self) -> String {
        format!("{} paddles gracefully across the pond", self.name)
    }
}

// =============================================================================
// 4. "INHERITANCE" via Composition + Trait Defaults
// =============================================================================
// Rust favors composition over inheritance

// Base "class" as a struct
struct AnimalBase {
    name: String,
    legs: u8,
}

impl AnimalBase {
    fn new(name: &str, legs: u8) -> Self {
        Self {
            name: name.to_string(),
            legs,
        }
    }
    
    fn walk(&self) -> String {
        format!("{} walks on {} legs", self.name, self.legs)
    }
}

// "Derived class" using composition
struct Horse {
    base: AnimalBase,       // Embed the base struct
    speed_mph: u32,
}

impl Horse {
    fn new(name: &str, speed_mph: u32) -> Self {
        Self {
            base: AnimalBase::new(name, 4),  // Horses have 4 legs
            speed_mph,
        }
    }
    
    // Delegate to base
    fn walk(&self) -> String {
        self.base.walk()
    }
    
    // Extended behavior
    fn gallop(&self) -> String {
        format!("{} gallops at {} mph!", self.base.name, self.speed_mph)
    }
}

impl Animal for Horse {
    fn speak(&self) -> String {
        String::from("Neigh!")
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
}

// =============================================================================
// DEMONSTRATION - Polymorphism in action
// =============================================================================

// Function accepting any Animal - POLYMORPHISM via trait objects
fn introduce_animal(animal: &dyn Animal) {
    println!("  {}", animal.describe());
}

// Function using generics - compile-time polymorphism (faster, no vtable)
fn make_sound<T: Animal>(animal: &T) {
    println!("  Sound: {}", animal.speak());
}

fn main() {
    println!("=".repeat(60));
    println!("RUST OOP CONCEPTS DEMONSTRATION");
    println!("=".repeat(60));
    
    // Create instances
    let mut dog = Dog::new("Rex", 5, "German Shepherd");
    let cat = Cat::new("Whiskers", true);
    let street_cat = Cat::new("Shadow", false);
    let duck = Duck::new("Donald");
    let horse = Horse::new("Spirit", 35);
    
    // ---------------------------------------------------------------------
    println!("\n1. ENCAPSULATION DEMO:");
    println!("-".repeat(40));
    
    println!("  Dog's age: {}", dog.get_age());
    dog.set_age(6);
    println!("  After birthday: {}", dog.get_age());
    dog.set_age(100);  // Invalid - won't change
    println!("  After invalid set (100): {}", dog.get_age());
    // println!("{}", dog.name);  // ERROR! Private field
    
    // ---------------------------------------------------------------------
    println!("\n2. ABSTRACTION DEMO (Traits define interface):");
    println!("-".repeat(40));
    
    // All animals implement the same interface
    println!("  {} speaks: {}", dog.name(), dog.speak());
    println!("  {} speaks: {}", cat.name(), cat.speak());
    
    // ---------------------------------------------------------------------
    println!("\n3. POLYMORPHISM DEMO (Same method, different behavior):");
    println!("-".repeat(40));
    
    // Using trait objects (dynamic dispatch)
    let animals: Vec<&dyn Animal> = vec![&dog, &cat, &street_cat, &duck, &horse];
    
    for animal in &animals {
        introduce_animal(*animal);
    }
    
    // ---------------------------------------------------------------------
    println!("\n4. POLYMORPHISM - Generic functions:");
    println!("-".repeat(40));
    
    make_sound(&dog);
    make_sound(&cat);
    make_sound(&duck);
    
    // ---------------------------------------------------------------------
    println!("\n5. COMPOSITION ('Inheritance' Rust-style):");
    println!("-".repeat(40));
    
    println!("  {}", horse.walk());      // Delegated to base
    println!("  {}", horse.gallop());    // Extended behavior
    
    // ---------------------------------------------------------------------
    println!("\n6. MULTIPLE TRAITS (Duck can Animal + Swimmer):");
    println!("-".repeat(40));
    
    println!("  {}", duck.speak());
    println!("  {}", duck.swim());
    
    // ---------------------------------------------------------------------
    println!("\n7. DEFAULT TRAIT METHODS ('Inherited' behavior):");
    println!("-".repeat(40));
    
    // describe() is defined once in trait, used by all
    println!("  {}", dog.describe());
    println!("  {}", horse.describe());
    
    println!("\n{}", "=".repeat(60));
    println!("KEY TAKEAWAYS:");
    println!("{}", "=".repeat(60));
    println!("• ABSTRACTION:   Traits define behavior contracts");
    println!("• ENCAPSULATION: Private fields + public getters/setters");
    println!("• POLYMORPHISM:  Trait objects (dyn) or generics");
    println!("• INHERITANCE:   Composition + default trait methods");
    println!("{}", "=".repeat(60));
}
