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


### Day 6 (Part 2): Destructuring Tuples
**Concept:** Destructuring allows you to "unpack" a tuple into separate variables in a single line of code. This is cleaner than accessing each item with `.0`, `.1`, etc.

**The Challenge**
Create a program that unpacks a tuple into named variables.
1. Create a file: `src/bin/day6/destructuring.rs`
2. Define a tuple: `let person_data = ("Alice", 30, 165.5);` (Name, Age, Height)
3. Use destructuring to create three variables at once:
   `let (name, age, height) = person_data;`
4. Print the `name`, `age`, and `height` variables separately to prove they are now independent values.

**Expected Output:**
> Name: Alice, Age: 30, Height: 165.5

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+person_data_tuple+%3D+%28%22Alice%22%2C+30%2C+165.6%29%3B%0D%0A++++println%21%28%22Value+of+person_data_tuple%3A+%7B%3A%3F%7D%22%2C+person_data_tuple%29%3B%0D%0A%0D%0A++++let+%28name%2C+age%2C+height%29+%3D+person_data_tuple%3B%0D%0A++++println%21%28%22Value+of+name%3A+%7B%7D%22%2C+name%29%3B%0D%0A++++println%21%28%22Value+of+age%3A+%7B%7D%22%2C+age%29%3B%0D%0A++++println%21%28%22Value+of+height%3A+%7B%7D%22%2C+height%29%3B%0D%0A%7D)
