use std::fmt;
use std::fmt::{Display};


trait SayHello {
    fn hello(
        &self
    );
}

impl SayHello for u8 {
    fn hello(
        &self
    ) {
        println!("Hello, 0");
    }
}

impl SayHello for f32 {
    fn hello(
        &self
    ) {
        println!("Hello, 0.0");
    }
}




#[derive(Debug, PartialEq)]
enum FileState {
    Opened,
    Closed,
}

/// Реализация типажа Display для типа FileState (позволяет форматировать в {})
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FileState::Opened => write!(f, "OPENED"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}



#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(filename: &str) -> File {
        File {
            name: filename.to_string(),
            data: vec![],
            state: FileState::Closed
        }
    }
}
/// Реализация типажа Display для типа File (позволяет форматировать в {})
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File {} state: {}", self.name, self.state)
    }
}

fn main() {
    let png_sprite = File::new("1.png");
    println!("State is: {}", png_sprite.state);
    println!("{}", png_sprite);

    // Использование различных реализаций типажа SayHello
    let var_1: u8 = 154;
    let var_2: f32 = 1.17;
    var_1.hello();
    var_2.hello();


    
}
