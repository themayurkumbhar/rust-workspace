### Day 4: Strict Math & Data Types
**Concept:** Rust is a strongly typed language. It does not allow implicit operations between different types (e.g., adding an Integer to a Float) to prevent precision errors. You must explicitly "cast" types to make them compatible.

**The Challenge**
Create a program that attempts to add an integer and a float.
1. Create a file: `src/bin/day4/strict_math.rs`
2. Declare an integer: `let x = 10;`
3. Declare a float: `let y = 2.5;`
4. Attempt to print `x + y` (observe the error).
5. Fix the error by casting the integer using `x as f64`.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+some_integer%3Ai64+%3D+123456%3B%0D%0A%0D%0A++++let+some_float+%3D+1.23%3B%0D%0A%0D%0A++++%2F%2F+let+summation+%3D+some_integer+%2B+some_float%3B+%2F%2F+will+show+error%3A+Cannot+add+%60f64%60+to+%60i32%60%5BE0369%5D%0D%0A++++%2F%2F+CAST+some_integer+to+f64+to+work.%0D%0A++++let+summation+%3D+some_integer+as+f64+%2B+some_float%3B%0D%0A%0D%0A++++println%21%28%22Value+of+summation%3A+%7B%7D%22%2C+summation%29%3B%0D%0A%7D)

---

### Day 4 (Part 2): Integer vs. Float Division
**Concept:** In Rust, the type of number determines how math works.
* **Integer Division (`5 / 2`):** Rust drops the decimal part completely (truncates), returning an integer (`2`).
* **Float Division (`5.0 / 2.0`):** Rust calculates the precise decimal result (`2.5`).

**The Challenge**
Create a program that demonstrates this difference.
1. Create a file: `src/bin/day4/division.rs`
2. Define a variable `let a = 5 / 2;`
3. Define a variable `let b = 5.0 / 2.0;`
4. Print both variables to see the difference.

**Expected Output:**
> Integer: 2
> Float: 2.5

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++%2F%2F+integers+division+behaviour%0D%0A++++let+int_division+%3D+9+%2F+2%3B%0D%0A++++println%21%28%22Value+of+integer+division%3A+%7B%7D%22%2C+int_division%29%3B+%2F%2F+drops+decimals%0D%0A%0D%0A++++%2F%2F+float+division+behaviour%0D%0A++++let+float_division+%3D+9.0+%2F+2.0%3B%0D%0A++++println%21%28%22Value+of+float+division%3A+%7B%7D%22%2C+float_division%29%3B+%2F%2F+keeps+decimals%0D%0A%7D)
