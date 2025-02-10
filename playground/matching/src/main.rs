

enum MaybeNumber {
    Number(i32),
    Empty
}

enum NumberOrError {
    Number(i32),
    Error(ErrorCode)
}

enum ErrorCode {
    TooBig,
    TooSmall
}

fn pos_less_than_ten(input: i32) -> NumberOrError {
    match input {
        0..10 => NumberOrError::Number(input),
        ..0 => NumberOrError::Error(ErrorCode::TooSmall),
        10.. => NumberOrError::Error(ErrorCode::TooBig)
    }
}

fn match_str() {
    let s = "RHello";
    match s {
        "Hello" => println!("Hello"),
        other if other.starts_with("R") => println!("Starts with R!!"),
        other if other.starts_with("X") => println!("Starts with X!!"),
        _ => println!("Other") // Игнорируем значение
    }
}

fn main() {

    match_str();
}



