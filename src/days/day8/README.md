### Day 8 (Part 1): The `loop` and `break`
**Concept:** A `loop` runs infinitely. We use the `break` keyword to exit the loop once a specific condition is met.

**The Challenge**
1. Create a file: `src/bin/day8/infinite_loop.rs`
2. Create a mutable variable `let mut count = 0;`.
3. Inside a `loop` block:
    - Increment `count` by 1.
    - Print: "Counting: [number]"
    - If `count` is equal to 5, `break` the loop.
4. After the loop, print "Done!".

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+mut+counter+%3D+0%3B%0D%0A++++loop+%7B%0D%0A++++++++counter+%2B%3D+1%3B%0D%0A++++++++println%21%28%22%7B%7D%22%2C+counter%29%3B%0D%0A++++++++if+counter+%3D%3D+9+%7B%0D%0A++++++++++++break%3B%0D%0A++++++++%7D%0D%0A++++%7D%0D%0A%7D%0D%0A)

---

### Day 8 (Part 2): Loop Expressions (Returning Values)
**Concept:** In Rust, `loop` can return a value. You can assign the result of a loop directly to a variable using `let result = loop { ... };`.

**The Challenge**
1. Create a file: `src/bin/day8/loop_return.rs`
2. Create `let mut counter = 0;`.
3. Create a variable `let result = loop { ... };`.
4. Inside the loop:
   - Add 1 to `counter`.
   - If `counter` reaches 10, `break` and return `counter * 2`.
5. Print: "The result is [result]" (Expected: 20).

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+mut+counter+%3D+0%3B%0D%0A%0D%0A++++let+result+%3D+loop+%7B%0D%0A++++++++counter+%2B%3D+1%3B%0D%0A++++++++if+counter+%3D%3D+10+%7B%0D%0A++++++++++++break+counter+*+2%3B%0D%0A++++++++%7D%0D%0A++++%7D%3B%0D%0A%0D%0A++++println%21%28%22The+result+is+%7B%7D%22%2C+result%29%3B%0D%0A%7D%0D%0A)
