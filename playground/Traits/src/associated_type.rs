trait HasId {
    const ID: usize;
}

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
            Err(_) => ConvResult::Err(U32Error)
        }
    }
}

impl HasId for u32 {
    const ID: usize = 77;
}

fn main() {
    println!("Ram ID is: {}", u32::ID);
}