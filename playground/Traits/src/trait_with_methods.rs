enum ConvResult<T, E> {
    Ok(T),
    Err(E),
}

struct U32Error;
struct MyIntError;

trait IntFromStr {
    type Err; //Ассоциированный тип
    fn from_str(s: &str) -> ConvResult<u32, Self::Err>;
}

impl IntFromStr for u32 {
    type Err = U32Error;

    fn from_str(s: &str) -> ConvResult<u32, Self::Err> {
        match s.parse() {
            Ok(i) => ConvResult::Ok(i),
            Err(_) => ConvResult::Err(U32Error),
        }
    }
}

trait MakeDefault {
    fn my_default() -> Self;
}

impl MakeDefault for u16 {
    fn my_default() -> Self {
        777
    }
}

impl MakeDefault for String {
    fn my_default() -> Self {
        String::from("Blank")
    }
}

fn main() {
    let my_number = u16::my_default();
    let my_string = String::my_default();

    println!("My number = {}, My string = {}", my_number, my_string);
}