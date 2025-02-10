use std::cmp::Ordering;

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
}

// Derive реализация работает по полям структур
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }
// /// Eq: a = a всегда верно (кроме f32, если он равен NaN)
// impl Eq for Point {}

// impl PartialOrd for Point {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match self.x.partial_cmp(&other.x)? {
//             Ordering::Equal => {}
//             ord => return Some(ord),
//         }

//         self.y.partial_cmp(&other.y)
//     }
// }

// Другой вариант
// impl PartialOrd for Point {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Point {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.x.cmp(&other.x) {
//             Ordering::Equal => {}
//             ord => return ord,
//         }

//         self.y.cmp(&other.y)
//     }
// }
