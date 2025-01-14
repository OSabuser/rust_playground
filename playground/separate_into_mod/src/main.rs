pub mod operations {
    pub fn operation_1() {
        println!("Operation 1!");
    }

    pub fn operation_2() {
        println!("Operation 2!");
    }
}

pub mod my_module_2 {
    use super::operations::{operation_1, operation_2};

    pub fn use_operations() {
        //super::operations::operation_1();
        //super::operations::operation_2();
        operation_1();
        operation_2();
    }
}

use my_module_2::use_operations;

fn main() {
    use_operations();
}
