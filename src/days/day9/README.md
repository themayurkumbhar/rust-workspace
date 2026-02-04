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
