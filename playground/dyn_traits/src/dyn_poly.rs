use std::result;

fn main() {
    let selected = select_printable();
    call_print(selected);

}

#[derive(PartialEq)]
enum SelectedType {
    Uint,
    Float,
    Bool
}
fn read_user_input() -> SelectedType {
    // User's input emulation
    SelectedType::Float
}

fn select_printable() -> Box<dyn Printable>  {
    
    let result = read_user_input();
    
    if result == SelectedType::Bool {
        Box::new(true)
    } else if result == SelectedType::Float {
        Box::new(3.14)
    } else {
        Box::new(256)
    }
}

fn call_print(object: Box<dyn Printable>) {
    object.print();
}

trait Printable {
    fn print(&self);
}

impl <T: Printable> Printable for Box<T> {
    fn print(&self) {
        self.as_ref().print();
    }
}

impl Printable for u32 {
    fn print(&self) {
        println!("Unsigned number: {}", self);
    }
}

impl Printable for f32 {
    fn print(&self) {
        println!("Float number: {}", self);
    }
}

impl Printable for bool {
    fn print(&self) {
        println!("Bool value: {}", self);
    }
}