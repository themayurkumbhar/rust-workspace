### Day 9 (Part 1): The `while` Loop
**Concept:** `while` loops are perfect when you want to repeat a block of code until a specific condition changes.

**The Challenge**
1. Create a file: `src/bin/day9/while_countdown.rs`
2. Create `let mut number = 5;`.
3. Use a `while number != 0` loop:
    - Print the current number.
    - Subtract 1 from the number (`number -= 1`).
4. After the loop, print "LIFTOFF! 🚀".

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+mut+some_countdown+%3D+10%3B%0D%0A%0D%0A++++while+some_countdown+%21%3D+0+%7B%0D%0A++++++++println%21%28%22%7B%7D%21%22%2C+some_countdown%29%3B%0D%0A++++++++some_countdown+-%3D1%3B%0D%0A++++%7D%0D%0A++++println%21%28%22Liftoff%21+%F0%9F%9A%80%22%29%3B%0D%0A%7D%0D%0A)

---

### Day 9 (Part 2): Manual Array Iteration
**Concept:** You can use a `while` loop and an index variable to move through an array.

**The Challenge**
1. Create a file: `src/bin/day9/while_array.rs`
2. Define an array: `let a = [10, 20, 30, 40, 50];`.
3. Create `let mut index = 0;`.
4. Use a `while index < 5` loop:
   - Print the element at `a[index]`.
   - Increment `index` by 1.

**Warning:** If your index goes to 5 or higher, the program will "panic" (crash). This is why `while` isn't the preferred way to loop over collections!

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+some_number_array%3A+%5Bi32%3B+5%5D+%3D+%5B10%2C+11%2C+12%2C+13%2C+14%5D%3B%0D%0A++++let+mut+index+%3D+0%3B%0D%0A%0D%0A++++while+index+%3C+some_number_array.len%28%29+%7B%0D%0A++++++++println%21%28%22Value+at+index%5B%7B%7D%5D%3A+%7B%7D%22%2C+index%2C+some_number_array%5Bindex%5D%29%3B%0D%0A++++++++index+%2B%3D+1%3B%0D%0A++++%7D%0D%0A%7D%0D%0A)
