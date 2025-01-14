use std::collections::HashMap;

macro_rules! my_vec {
    ($($item:expr),*) => {
        {
            let mut vec = Vec::new();
            $(vec.push($item);)*
            vec
        }
    };
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut custom_map = HashMap::new();
            $(
                custom_map.insert($key, $value);
            )*
            custom_map
        }
    };
}

fn main() {
    let my_map = hashmap!(1 => vec![123, 15], 2 => vec![14, 11]);
    println!("MyMap: {:?}", my_map);
}