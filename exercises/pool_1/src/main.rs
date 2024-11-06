
fn is_int_less_than_zero(number: i32) -> () {
    
    match number < 0 {
        true => println!("Отрицательное!"),
        false => println!("Положительное!"),
    };
}


fn main() {
    let num = -123;
    is_int_less_than_zero(num);
}
