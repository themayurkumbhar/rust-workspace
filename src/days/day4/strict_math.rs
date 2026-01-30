fn main() {
    let some_integer:i64 = 123456;

    let some_float = 1.23;

    // let summation = some_integer + some_float; // will show error: Cannot add `f64` to `i32`[E0369]
    // CAST some_integer to f64 to work.
    let summation = some_integer as f64 + some_float;

    println!("Value of summation: {}", summation);
}