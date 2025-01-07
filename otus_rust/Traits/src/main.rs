trait Pet: Animal {
    fn say() {
        println!("Hey {}!", Self::species_name());
    }

    fn call(&self);
}

trait Animal {
    fn species_name() -> String;
}

struct Cat {
    name: String,
}
impl Pet for Cat {
    fn say() {
        println!("{} says meow", Self::species_name());
    }
    fn call(&self) {
        println!("Hello, {}", self.name);
    }
}
impl Animal for Cat {
    fn species_name() -> String {
        String::from("Cat")
    }
}
struct Dog {
    name: String,
}
impl Pet for Dog {
    fn say() {
        println!("{} says woof", Self::species_name());
    }
    fn call(&self) {
        println!("Hello, {}", self.name);
    }
}
impl Animal for Dog {
    fn species_name() -> String {
        String::from("Dog")
    }
}
enum MyPets {
    Dog(Dog),
    Cat(Cat),
    Other
}

impl Pet for MyPets {
    fn call(&self) {
        match self {
            MyPets::Cat(dog) => dog.call(),
            MyPets::Dog(cat) => cat.call(),
            MyPets::Other => println!("What are you??!")

        }
    }
}
fn main() {
    let my_cat = Cat {
        name: String::from("Kitty")
    };
    let my_dog = Dog {
        name: String::from("Obschy")
    };
    my_cat.call();
    my_dog.call();

    //let my_pets = MyPets::Dog(my_dog);
   // my_pets.call();
}
