use core::fmt;

struct Cat {
    name: String,
    age: u8,
}

// #[derive(Debug)] is used to automatically implement the Debug trait for the struct.
// Выводит все поля структуры в формате Debug - это не всегда удобно.
fn main() {
    let cat = Cat {
        name: "Buster".to_string(),
        age: 10,
    };

    println!("Display output: {}", cat); // String formatting
    println!("Debug output: {cat:#?}"); // Debug formatting

    // Converting to string (using Display trait)
    let cat_str_info = cat.to_string();
    println!("String output: {}", cat_str_info);
   
}

// Implementing the Display trait for Cat
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cat: {} is {} years old", self.name, self.age)
    }
}   

// Implementing the Debug trait for Cat
impl fmt::Debug for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "Cat {{ name: {}, age: {} }}", self.name, self.age)
        f.debug_struct("Cat")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}