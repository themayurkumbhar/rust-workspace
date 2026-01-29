fn main() {
    let num_value = 123;
    println!("Value before shadowing: {}", num_value);

    let num_value = 456;
    println!("Value after shadowing: {}", num_value);

    let num_value = num_value + 1;
    println!("Value after shadowing and addition: {}", num_value);

    let num_value = num_value * 2;
    println!("Value after shadowing and multiplication: {}", num_value);
}