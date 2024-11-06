//#1
fn is_int_less_than_zero(number: i32) -> () {
    match number < 0 {
        true => println!("Отрицательное!"),
        false => println!("Положительное!"),
    };
}
//#2
fn show_string_length(input: &str) {
    println!("String length is: {}", input.len());
}
//#3
fn show_the_last_symbol(input: &str) {
    match input.get(input.len() - 1..) {
        Some(x) => println!("The last char is: {x}"),
        None => println!("Whoops!"),
    };
}
//#4
fn is_first_chars_the_similar(input_1: &str, input_2: &str) -> bool {
    input_1.starts_with(&input_2[..1])
}

//#5
fn get_the_magic_char(input: &str) -> char {
    match input.get(input.len() - 1..) {
        'ь' => 
        None => '-',
    }
    
}
fn main() {
    let num = -123;
    is_int_less_than_zero(num);

    let txt: &str = "abcde";
    show_string_length(txt);
    show_the_last_symbol(txt);

    println!("Result #4 is: {}", is_first_chars_the_similar("abc", "ade"));
}
