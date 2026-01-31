### Day 5 (Part 1): Booleans & Conditionals
**Concept:** Booleans (`bool`) store `true` or `false`. They are the heartbeat of control flow (if/else).

**The Challenge**
Create a program that makes a simple decision based on a boolean variable.
1. Create a file: `src/days/day5/booleans.rs`
2. Define a variable: `let is_rust_fun = true;`
3. Write an `if` statement checking that variable.
    - If true: print "It is fun!"
    - If false: print "It is hard!"
4. (Optional) Change the value to `false` and run it again to see the other message.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+is_rus_fun+%3D+true%3B%0D%0A++++%0D%0A++++if+is_rus_fun+%7B%0D%0A++++++++println%21%28%22It+is+Fun%21%22%29%3B%0D%0A++++%7D+else+%7B%0D%0A++++++++println%21%28%22It+not+Fun%21%22%29%3B%0D%0A++++%7D%0D%0A%7D)

---

### Day 5 (Part 2): Characters vs. Strings
**Concept:**
* **String (`" "`):** Uses double quotes. A collection of characters.
* **Char (`' '`):** Uses **single quotes**. In Rust, a `char` is 4 bytes and can represent Unicode (like Emojis 🦀).

**The Challenge**
Create a program that prints specific characters.
1. Create a file: `src/days/day5/chars.rs`
2. Define a generic letter: `let letter = 'A';` (Note the single quotes!)
3. Define a number character: `let number_char = '9';`
4. Define an emoji character: `let emoji = '🚀';`
5. Print all three.

**Constraint:** You *must* use single quotes. Double quotes will create a String, which is a different type!

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A%0D%0A++++let+some_letter_char+%3D+%27A%27%3B%0D%0A++++println%21%28%22Value+of+some_letter_char+is%3A+%7B%7D%22%2C+some_letter_char%29%3B%0D%0A%0D%0A++++let+some_number_char+%3D+%277%27%3B%0D%0A++++println%21%28%22Value+of+some_number_char+is%3A+%7B%7D%22%2C+some_number_char%29%3B%0D%0A%0D%0A++++let+some_emoji_char+%3D+%27%F0%9F%A6%80%27%3B%0D%0A++++println%21%28%22Value+of+some_emoji_char+is%3A+%7B%7D%22%2C+some_emoji_char%29%3B%0D%0A%7D)