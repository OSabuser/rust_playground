fn main() {
    println!("Hello, world!");

    let a: u32 = 123;
    let b: f32 = 123.123;
    let c: bool = false;

    a.print();
    b.print();
    c.print();

    call_print(a);
    call_print(b);
    call_print(c);
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