trait Pet {

    type Toy: Describe;

    fn get_toy(&self) -> Self::Toy;

    fn say_hello();

    fn say_test(&self) {
        println!("Test!!");
    }
}

struct Cat {
    age: u8,
}
struct Dog {
    bark: u8,
}

struct CatToy;

impl Pet for Cat {

    type Toy = CatToy;

    fn get_toy(&self) -> Self::Toy {
        CatToy
    }
    fn say_hello() {
        println!("Meoooow!1");
    }

    fn say_test(&self) {
        println!("Test: {}", self.age);
    }
}

struct DogToy;
impl Pet for Dog {
    type Toy = DogToy;

    fn get_toy(&self) -> Self::Toy {
        DogToy
    }

    fn say_hello() {
        println!("Barrrk!2");
    }

    fn say_test(&self) {
        println!("Test: {}", self.bark);
    }
}
impl Describe for DogToy {
    fn describe(&self) -> String {
        String::from("Bone")
    }
}

impl Describe for CatToy {
    fn describe(&self) -> String {
        String::from("Mouse")
    }
}

trait Describe {
    fn describe(&self) -> String;
}

fn play_with_pet<T: Pet>(exe: &T) {
    println!("Calling a pet...");
    T::say_hello();

    exe.say_test()

    let toy = exe.get_toy();

    println!("Playing with pet's toy: {}", toy.describe());
}

fn main() {}