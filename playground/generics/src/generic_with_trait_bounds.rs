trait Pet {
    fn say_hello();

    fn say_test(&self) {
        println!("Test!!");
    }
}

struct Cat {
    age: u8,
}
impl Pet for Cat {
    fn say_hello() {
        println!("Meoooow!1");
    }

    fn say_test(&self) {
        println!("Test: {}", self.age);
    }
}

fn play_with_pet<T: Pet>(exe: &T) {
    println!("Calling a pet...");
    T::say_hello();

    exe.say_test()
}

fn main() {}