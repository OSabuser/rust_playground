//https://code.mu/ru/rust/tasker/stager/1/1/

//#1
#[allow(dead_code)]
fn is_int_less_than_zero(number: i32) -> () {
    match number < 0 {
        true => println!("Отрицательное!"),
        false => println!("Положительное!"),
    };
}
//#2
#[allow(dead_code)]
fn show_string_length(input: &str) {
    println!("String length is: {}", input.len());
}
//#3
#[allow(dead_code)]
fn show_the_last_symbol(input: &str) {
    match input.get(input.len() - 1..) {
        Some(x) => println!("The last char is: {x}"),
        None => println!("Incorrect string!"),
    };
}
//#4
#[allow(dead_code)]
fn is_first_chars_the_similar(input_1: &str, input_2: &str) -> bool {
    input_1.starts_with(&input_2[..1])
}

//#5
#[allow(dead_code)]
fn get_the_magic_char(input: &str) -> () {
    println!("Input string is: {input}");
    let chars_array = input.chars().rev();

    let mut is_match_found = false;

    for symbol in chars_array {
        if symbol == 'ь' && !is_match_found {
            is_match_found = true;
            continue;
        } else {
            println!("{}", symbol);
            break;
        }
    }
}

//#5
#[allow(dead_code)]
fn convert_int_to_string(value: i32) -> String {
    value.to_string()
}

//#6
#[allow(dead_code)]
fn check_if_number_in_range(number: u16, range: (u16, u16)) -> bool {
    number <= range.1 && number >= range.0
}

//https://code.mu/ru/rust/tasker/stager/1/2/
//#1
#[allow(dead_code)]
fn show_the_least_digit_of_number(number: i32) {
    println!("The least digit is: {}", number % 10);
}
//#2
#[allow(dead_code)]
fn show_the_most_digit_of_number(number: i32) {
    match number.to_string().get(..1) {
        Some(x) => println!("The most digit is: {}", x),
        None => println!("Error!"),
    }
}

fn main() {
    show_the_least_digit_of_number(123);
    show_the_most_digit_of_number(123);
}
