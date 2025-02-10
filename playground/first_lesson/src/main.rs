enum Color {
    Red,
    Green,
    Blue,
}

enum MaybeNumber {
    Number(i32),
    Empty
}

enum MaybeNumberNamed {
    Number {
        value: i32
    },
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
    if input < 10 && input >= 0{
        NumberOrError::Number(input)
    } else if input < 0{
         NumberOrError::Error(ErrorCode::TooSmall)
    } else {
        NumberOrError::Error(ErrorCode::TooBig)
    }
}

fn try_process_number(number: NumberOrError) {
    match number {
        NumberOrError::Number(value) => println!("Number: {}", value),
        NumberOrError::Error(ErrorCode::TooBig) => {
            println!("Value is too big!")
        },
        NumberOrError::Error(ErrorCode::TooSmall) => println!("Value is too small")
    }
}


enum MultiColor {
    RgbInt(u8, u8, u8),
    RgbFloat(f32, f32, f32),
    IntArr([u8; 3]),
    FloatArr([f32; 3])
}

enum Empty {}

fn main() {

    let val_or_err = pos_less_than_ten(5);
    try_process_number(val_or_err);

    let val_or_err = pos_less_than_ten(15);
    try_process_number(val_or_err);

    let val_or_err = pos_less_than_ten(-1);
    try_process_number(val_or_err);
    /* 
    let maybe_number = try_get_number(5);
    let maybe_number_named = try_get_number_named(5);
    match maybe_number {
        MaybeNumber::Number(val) => println!("Number: {}", val),
        MaybeNumber::Empty => println!("Nothing"),
        
    }
    match maybe_number_named {
        MaybeNumberNamed::Number {value} => println!("Number: {}", value),
        MaybeNumberNamed::Empty => println!("Nothing"),
        
    }

    if let MaybeNumber::Number(value) = maybe_number {
        println!("Number in if let: {}", value);
    }

    while let MaybeNumber::Number(value) = maybe_number {
        println!("Number in while let: {}", value);
    }
*/



}



fn try_get_number(input: i32) -> MaybeNumber {
    if input < 10 {
        MaybeNumber::Number(input)
    } else {
        MaybeNumber::Empty
    }
}

fn try_get_number_named(input: i32) -> MaybeNumberNamed {
    if input < 10 {
        MaybeNumberNamed::Number { value: input }
    } else {
        MaybeNumberNamed::Empty
    }
}