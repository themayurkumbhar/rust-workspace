fn main() {
    const SPEED: i32 = 100;

    println!("Value of Constant SPEED: {}", SPEED);

    const PI: f32 = 3.14;

    println!("Value of Constant PI: {}", PI);

    /*
    if we change constant value like
        SPEED = 200
    Compiler will complaint "cannot assign to this expression"
     */
}