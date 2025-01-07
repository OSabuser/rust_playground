use std::result;

fn call_print<T: Printable +?Sized>(t: &T) {
    t.print();
}

fn call_print_dyn(t: &dyn Printable) {
    t.print();
}

// ! Sized types
// [T]
// &str
// dyn Trait
fn main() {
  let b = 1.32;
  let dyn_b: &dyn Printable = &b;

  call_print(dyn_b);
  call_print_dyn(dyn_b);
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
impl Printable for f64 {
    fn print(&self) {
        println!("Float number: {}", self);
    }
}
impl Printable for bool {
    fn print(&self) {
        println!("Bool value: {}", self);
    }
}
