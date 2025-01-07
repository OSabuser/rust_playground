fn my_drop<T>(x: T) {

}



fn main() {
    let x = 1;
    let y = String::from("Hello!");
    let z = (1, 2, 3 , 4);
    my_drop(x);
    my_drop(y);
    my_drop(z);
}
