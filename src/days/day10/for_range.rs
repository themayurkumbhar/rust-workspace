fn main() {
    // generate incremental sequence
    for element in 1..5 {
        println!("Element in ascending array: {}", element);
    }
    println!();
    // generate reverse sequence
    for element in (1..5).rev() {
        println!("Element in descending array: {}", element);
    }
}
