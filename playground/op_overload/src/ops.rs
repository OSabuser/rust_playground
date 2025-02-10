use std::ops::{Add, AddAssign, Mul, Neg};

fn main() {
    let mut p1 = Point { x: 1, y: 2 };
    let mut p2 = Point { x: 3, y: 4 };

    let p_sum = p1 + p2;

    println!("p1 + p2 = {:?}", p_sum);

    let p_sum = p1 + 3_i32;

    println!("p1 + 3 = {:?}", p_sum);

    let mut p_3 = -p1;

    println!("-p1 = {:?}", p_3);

    p_3 += p1;

    println!("p3 += p1 = {:?}", p_3);
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

///
/// Реализация оператора -
///
impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

///
/// Реализация оператора + между Point & i32
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Self::Output {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

///
/// Реализация оператора + между i32 & Point
/// (Симметричное сложение: a + b = b + a)
///
impl Add<Point> for i32 {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        other + self
    }
}

//
/// Реализация оператора + между &Point (нет неявного копирования)
impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

///
/// Реализация оператора + между Point
///
impl Add<Self> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Derive реализация работает по полям структур
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

///
/// Реализация оператора * между Point
///
impl Mul<Point> for Point {
    type Output = i32;

    fn mul(self, other: Point) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}
