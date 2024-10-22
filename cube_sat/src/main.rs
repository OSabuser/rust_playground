//! Имитация работы системы спутниковой связи

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BaseStation {
    radio_frequency: u64,
}

fn main() {
    let base = Rc::new(RefCell::new(BaseStation {
        radio_frequency: 1_000_000,
    }));

    println!("#1 Base frequency is: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_frequency = 32768;
        println!("[] Base frequency is: {:?}", base_2);
    }

    println!("#2 Base frequency is: {:?}", base);
}
