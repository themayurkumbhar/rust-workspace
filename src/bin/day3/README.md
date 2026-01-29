## 🧪 Program 1: The Shadowing

Create a program demonstrating shadowing by changing a variable's type.
Context: Reuse a variable name to convert text to a number.

### Task:

1. Create src/bin/day3/shadowing_types.rs

2. Declare 'spaces' as a string " ".

3. Shadow 'spaces' to store its length (integer).

4. Print the integer.

`Constraint: Do NOT use mut.`



[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+spaces+%3D+%22this+is+string.%22%3B%0D%0A%0D%0A++++%2F%2F+Shadowing+the+spaces+var+here%0D%0A++++let+spaces+%3D+spaces.len%28%29%3B%0D%0A%0D%0A++++println%21%28%22Value+of+shadowing+spaces%3A+%7B%7D%22%2C+spaces%29%3B%0D%0A%0D%0A%7D)

---

## Objective: Demonstrate variable shadowing to transform values.

### Task:

1. Create src/bin/day3/shadowing_values.rs. 
2. Start with let x = 5. 
3. Shadow with let x = x + 1. 
4. Shadow with let x = x * 2. 
5. Print x.

`Constraints: Do NOT use mut. Explain why this works in comments.`

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+spaces+%3D+%22this+is+string.%22%3B%0D%0A%0D%0A++++%2F%2F+Shadowing+the+spaces+var+here%0D%0A++++let+spaces+%3D+spaces.len%28%29%3B%0D%0A%0D%0A++++println%21%28%22Value+of+shadowing+spaces%3A+%7B%7D%22%2C+spaces%29%3B%0D%0A%0D%0A%7D)