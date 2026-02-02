fn main() {
    // [i32; 5] defines type and size of array.
    let some_int_array: [i32; 5] = [1,2,3,4,5];

    println!("Value of index[0]: {}", some_int_array[0]);
    println!("Value of index[4]: {}", some_int_array[4]);
}