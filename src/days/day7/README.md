### Day 7 (Part 1): Array Basics
**Concept:** An Array is a collection of elements of the **same type** with a **fixed length**.
* **Type Signature:** `[i32; 4]` means "An array of 4 integers."
* **Indexing:** Access elements using square brackets `[0]`.

**The Challenge**
Create a program that defines an array and retrieves data from it.
1. Create a file: `src/bin/day7/array_basics.rs`
2. Define an array: `let lucky_numbers = [7, 42, 13, 99];`
3. Print the **first** number using index `[0]`.
4. Print the **last** number using index `[3]`.

**Constraint:** Do not try to access index `[4]` or higher (this will crash the program!).

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++%2F%2F+%5Bi32%3B+5%5D+defines+type+and+size+of+array.%0D%0A++++let+some_int_array%3A+%5Bi32%3B+5%5D+%3D+%5B1%2C2%2C3%2C4%2C5%5D%3B%0D%0A%0D%0A++++println%21%28%22Value+of+index%5B0%5D%3A+%7B%7D%22%2C+some_int_array%5B0%5D%29%3B%0D%0A++++println%21%28%22Value+of+index%5B4%5D%3A+%7B%7D%22%2C+some_int_array%5B4%5D%29%3B%0D%0A%7D)

---

### Day 7 (Part 2): The Debug Trait & Printing
**Concept:** Unlike simple scalars (integers/bools), compound types like Arrays do not implement the `Display` trait. To see what's inside an array, we must use the **Debug** formatter `{:?}`.

**The Challenge**
1. Create a file: `src/bin/day7/debug_print.rs`
2. Define an array of strings: `let languages = ["Rust", "Python", "Go"];`
3. Print the array using the standard debug formatter `{:?}`.
4. Print it again using the "pretty-print" formatter `{:#?}`.

**Why this matters:**
Rust is explicit. It doesn't assume how you want to show a collection to a user, so it forces you to use developer-centric formatting for quick inspection.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+some_string_array+%3D+%5B%22Red%22%2C+%22Green%22%2C+%22Blue%22%2C+%22Yellow%22%5D%3B%0D%0A++++%0D%0A++++%2F%2F+single+line+print%0D%0A++++println%21%28%22Value+of+some_string_array+on+single+line%3A+%7B%3A%3F%7D%22%2C+some_string_array%29%3B%0D%0A++++%0D%0A++++%2F%2F+multi+line+print%0D%0A++++println%21%28%22Value+of+some_string_array+on+multiple+lines%3A+%7B%3A%23%3F%7D%22%2C+some_string_array%29%3B%0D%0A%7D)