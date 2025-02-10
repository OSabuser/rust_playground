
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn show_coord(&self) {
        println!("X:{}, Y:{}", self.x, self.y);
    }
}


struct Color(u8, u8, u8, u8);


fn use_point_x_coord(Point {x, ..}: Point) {
    println!("X_COORD: {x}");
}

fn main() {
    let point = Point {
        x: 1,
        y: 2
    };

    let point_2 = Point::new(13, 14);

    point_2.show_coord();


    use_point_x_coord(point);
    let red_color = Color(0xFF, 0xFF, 0x00, 0x00);

    let Color(a, r, g, b) = red_color;

    println!("Opacity: {:?}", red_color.0);
    //println!("X_POS: {:?}", point.x);
}
