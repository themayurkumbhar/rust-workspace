## Day 10: The "Golden Standard" (for Loops)

In Rust, `for` loops are the most common way to iterate. They are safer and faster than `while` loops because they eliminate manual indexing and prevent "index out of bounds" errors.

### Program 1: Safer Array Iteration
**Concept:** Instead of using an index (like `a[i]`), we iterate directly over the elements of the collection.

**The Challenge:**
1. Create `src/bin/day10/for_array.rs`.
2. Define an array: `let a = [10, 20, 30, 40, 50];`.
3. Use `for element in a` to print each value.

[🚀 Run Day 10 in Rust Playground](<YOUR_PLAYGROUND_LINK_HERE>)

### Program 2: Ranges & Reversing
**Concept:** Rust can generate a sequence of numbers on the fly using Ranges.
- `(1..4)` generates `1, 2, 3` (Exclusive of the final number).
- `.rev()` reverses the sequence.

**The Challenge:**
1. Create `src/bin/day10/for_range.rs`.
2. Use `for number in (1..4).rev()` to print a 3-2-1 countdown.
3. Print "LIFTOFF! 🚀" at the end.

[🚀 Run Day 10 in Rust Playground](<YOUR_PLAYGROUND_LINK_HERE>)
---
