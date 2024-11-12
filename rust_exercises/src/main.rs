//https://code.mu/ru/rust/tasker/stager/1/1/

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
        None => println!("Incorrect string!"),
    };
}
//#4
fn is_first_chars_the_similar(input_1: &str, input_2: &str) -> bool {
    input_1.starts_with(&input_2[..1])
}

//#5
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
fn convert_int_to_string(value: i32) -> String {
    value.to_string()
}


//#6
fn check_if_number_in_range(number: u16, range: (u16, u16)) -> bool {
    
    number <= range.1 && number >= range.0

}


fn main() {
    let num = -123;
    is_int_less_than_zero(num);

    let txt: &str = "abcde";
    show_string_length(txt);
    show_the_last_symbol(txt);

    println!("Result #4 is: {}", is_first_chars_the_similar("abc", "ade"));

    get_the_magic_char("Selamin!ь");

    println!("Converted val is {}", convert_int_to_string(123));


    let value = 17;
    let range = (1, 16);
    println!("Is {value} in : {}", check_if_number_in_range(value, range));
}
