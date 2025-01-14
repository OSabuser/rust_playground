/// # Замыкания  
/// ## 


fn main() {

    let var: Vec<u32> = (1..5).into_iter().map(|x| x*7).collect();

    println!("New vec is: {:?}", var);
    
}
