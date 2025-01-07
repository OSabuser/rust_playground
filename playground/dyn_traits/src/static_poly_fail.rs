fn main() {
    let selected = select_printable();
    call_print(selected);

}

enum SelectedType {
    Uint,
    Float,
    Bool
}
fn read_user_input() -> SelectedType {
    // User's input emulation
    SelectedType::Bool
}

fn select_printable() -> impl Printable {
    
    let result = match read_user_input() {
        SelectedType::Bool => true,
        SelectedType::Float => 3.14,
        SelectedType::Uint => 256,
    };

    result
}

fn call_print<T: Printable>(object: T) {
    object.print();
}

trait Printable {
    fn print(&self);
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