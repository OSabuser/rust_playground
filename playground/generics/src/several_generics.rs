enum MyResult<T,E> {
    Ok(T),
    Err(E)
}

struct U32Error;

fn from_string(s: &str) -> MyResult<u32, U32Error> {
    match s {
        "123" => MyResult::Ok(123),
        "asd" => MyResult::Err(U32Error),
        _ => MyResult::Err(U32Error)
        
    }
}

struct GetDataError;

fn from_lan() -> MyResult<[u8;256], GetDataError> {
    MyResult::Ok([0;256])
}


fn main() {
    
}