fn main() {
    let some_number_array: [i32; 5] = [10, 11, 12, 13, 14];
    let mut index = 0;

    while index < some_number_array.len() {
        println!("Value at index[{}]: {}", index, some_number_array[index]);
        index += 1;
    }
}
