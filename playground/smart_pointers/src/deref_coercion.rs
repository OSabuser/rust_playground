#[derive(Debug)]
struct SortedVec<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    fn new() -> SortedVec<T> {
        SortedVec { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.data.sort();
    }
}

impl<T: Ord> std::ops::Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        println!("I'm here!!!");
        &self.data
    }
}
fn main() {
    let mut first_vector = SortedVec::new();
    first_vector.push(1);
    first_vector.push(21);
    first_vector.push(510);
    first_vector.push(1);
    first_vector.push(13);
    first_vector.push(113);

    println!("First vector: {:?}", first_vector);

    // SortedVec не реализует Clone => согласно deref coercions используется clone типа Vec
    // second_vector становится Vec<i32>, при push не произойдет сортировка
    let mut second_vector = first_vector.clone(); // Vec<i32>
                                                  //let mut second_vector: SortedVec<i32> = first_vector.clone() - ошибка!
    println!("Second vector: {:?}", second_vector);

    second_vector.push(7);
    println!("Second vector after push: {:?}", second_vector);
}
