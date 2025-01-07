use std::str::FromStr;

trait MyTrait {

}

struct MyStruct {}

impl FromStr for MyStruct {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct {  })
    }
    
}

impl MyTrait for u32 {}

impl FromStr for bool { // Error: нельзя реализовывать чужие трейты для чужих типов (ORPHAN RULE!)
    
}

fn main() {
    
}