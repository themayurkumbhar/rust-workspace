## Day 10: The "Golden Standard" (for Loops)

In Rust, `for` loops are the most common way to iterate. They are safer and faster than `while` loops because they eliminate manual indexing and prevent "index out of bounds" errors.

### Program 1: Safer Array Iteration
**Concept:** Instead of using an index (like `a[i]`), we iterate directly over the elements of the collection.

**The Challenge:**
1. Create `src/bin/day10/for_array.rs`.
2. Define an array: `let a = [10, 20, 30, 40, 50];`.
3. Use `for element in a` to print each value.

[🚀 Run Day 10 in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+some_array+%3D+%5B%273%27%2C+%274%27%2C+%275%27%2C+%276%27%2C+%277%27%2C+%278%27%2C+%279%27%5D%3B%0D%0A%0D%0A++++for+element+in+some_array.iter%28%29+%7B%0D%0A++++++++println%21%28%22Element+in+some_array%3A+%7B%7D%22%2C+element%29%3B%0D%0A++++%7D%0D%0A%7D)

### Program 2: Ranges & Reversing
**Concept:** Rust can generate a sequence of numbers on the fly using Ranges.
- `(1..4)` generates `1, 2, 3` (Exclusive of the final number).
- `.rev()` reverses the sequence.

**The Challenge:**
1. Create `src/bin/day10/for_range.rs`.
2. Use `for number in (1..4).rev()` to print a 3-2-1 countdown.
3. Print "LIFTOFF! 🚀" at the end.

[🚀 Run Day 10 in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++%2F%2F+generate+incremental+sequence%0D%0A++++for+element+in+1..5+%7B%0D%0A++++++++println%21%28%22Element+in+ascending+array%3A+%7B%7D%22%2C+element%29%3B%0D%0A++++%7D%0D%0A++++println%21%28%29%3B%0D%0A++++%2F%2F+generate+reverse+sequence%0D%0A++++for+element+in+%281..5%29.rev%28%29+%7B%0D%0A++++++++println%21%28%22Element+in+descending+array%3A+%7B%7D%22%2C+element%29%3B%0D%0A++++%7D%0D%0A%7D%0D%0A)

---
