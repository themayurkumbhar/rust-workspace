fn main() {
    let spaces = "this is string.";

    // Shadowing the spaces var here
    let spaces = spaces.len();

    println!("Value of shadowing spaces: {}", spaces);

}