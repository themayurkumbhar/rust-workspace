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