### Day 6 (Part 1): Tuple Basics
**Concept:** A Tuple is a fixed-length group of values that can have **different types**.
* **Creation:** `let my_tuple = (500, 6.4, "Hello");`
* **Access:** You access elements using a dot and the index: `my_tuple.0` gets `500`.

**The Challenge**
Create a program that groups mixed data types.
1. Create a file: `src/bin/day6/tuple_basics.rs`
2. Define a tuple named `my_tuple` containing three elements:
    - An integer: `500`
    - A float: `6.4`
    - A string literal: `"Day 6"`
3. Print the second element (`6.4`) using `my_tuple.1`.
4. Print the third element (`"Day 6"`) using `my_tuple.2`.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+tuple_of_three+%3D+%28123%2C+3.14%2C+%22Hello+Rust%21%22%29%3B%0D%0A%0D%0A++++println%21%28%22Value+of+tuple_of_three.0%3A+%7B%7D%22%2C+tuple_of_three.0%29%3B%0D%0A++++println%21%28%22Value+of+tuple_of_three.1%3A+%7B%7D%22%2C+tuple_of_three.1%29%3B%0D%0A++++println%21%28%22Value+of+tuple_of_three.2%3A+%7B%7D%22%2C+tuple_of_three.2%29%3B%0D%0A%7D)
