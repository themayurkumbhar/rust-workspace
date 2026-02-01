fn main() {
    let person_data_tuple = ("Alice", 30, 165.6);
    println!("Value of person_data_tuple: {:?}", person_data_tuple);

    let (name, age, height) = person_data_tuple;
    println!("Value of name: {}", name);
    println!("Value of age: {}", age);
    println!("Value of height: {}", height);
}