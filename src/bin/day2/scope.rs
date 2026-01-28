fn main() {
    let outer_variable = 100;

    {
        let inner_variable = 200;
        println!("Value inside block- outer_variable: {}", outer_variable);
        println!("inner_variable: {}", inner_variable);
    }

    println!("Value outside block- outer_variable: {}", outer_variable);
    /*println!("Value outside block- outer_variable: {}", inside_variable);
                                                         ^^^^^^^^^^^^^^^ not found in this scope
    */
}