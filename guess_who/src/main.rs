extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
   
    println!("Try to guess the number!");

    loop {
        println!("Type your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .ok()
            .expect("Read operation has been failed!");

        // Затенение guess: String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Мало!"),
            Ordering::Greater => println!("Много!"),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            } 
        }
    }
}
