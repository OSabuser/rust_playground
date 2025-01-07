use std::result;


fn print_slice<T: Printable>(slice: &[T]) {
    for item in slice {
        item.print();
    }
}
fn print_slice_boxed(slice: &[Box<dyn Printable>]) {
    for item in slice {
        item.print();
    }
}

fn print_slice_of_refs(slice: &[&dyn Printable]) {
    for item in slice {
        item.print();
    }
}


fn main() {
    let selected = select_printable();
    call_print(selected);

    let slice_1= [1.23, 1.45, 44.0];
    let slice_2 = [false, true, true];
  
    let mixed_array_boxed: [Box<dyn Printable>; 3] = [Box::new(true), Box::new(1), Box::new(1.24)];
    let mixed_array_refs: [&dyn Printable; 3]= [&true, &1, &1.24];

    println!("Print the boxed trait objects!");
    print_slice_boxed(&mixed_array_boxed);
    println!("Print the refs to trait objects!");
    print_slice_of_refs(&mixed_array_refs);
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
