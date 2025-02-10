fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    if p1 == p2 {
        println!("p1 == p2 !!");
    } else {
        println!("p1 != p2 !!");
    }

    if p1 > p2 {
        println!("p1 > p2 !!");
    } else {
        println!("p1 <= p2 !!");
    }

    let p3 = Point { x: 0, y: 0 };

    if p3 == 0_i32 {
        println!("p3 == 0 !!");
    }
}

// Derive реализация работает по полям структур
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq<i32> for Point {
    fn eq(&self, other: &i32) -> bool {
        self.x == *other && self.y == *other
    }
}
