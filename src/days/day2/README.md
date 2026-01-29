## 🧪 Program 1: The Magic Number (Constants)

### Task:

1. Define a constant named `MAX_SPEED` inside main (or above it) and set it to `100`.

2. Remember: You must specify the type (e.g., `const MAX_SPEED: i32 = 100;`).

3. Define a constant named `PI` and set it to `3.14`.

4. Print both values clearly.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++const+SPEED%3A+i32+%3D+100%3B%0D%0A%0D%0A++++println%21%28%22Value+of+Constant+SPEED%3A+%7B%7D%22%2C+SPEED%29%3B%0D%0A%0D%0A++++const+PI%3A+f32+%3D+3.14%3B%0D%0A%0D%0A++++println%21%28%22Value+of+Constant+PI%3A+%7B%7D%22%2C+PI%29%3B%0D%0A%0D%0A++++%2F*%0D%0A++++if+we+change+constant+value+like%0D%0A++++++++SPEED+%3D+200%0D%0A++++Compiler+will+complaint+%22cannot+assign+to+this+expression%22%0D%0A+++++*%2F%0D%0A%7D)

---


## 🧪 Program 2: The Secret Room (Scope)

In Rust, variables only "live" inside the block {} where they are created. This is called Scope.

### Task:

1. In main, create a variable `let outer_variable = 10;`.

2. Start a new block by opening a `curly brace {`.

3. Inside that block, create `let inner_variable = 20;`.

4. Print both variables inside the block.

5. Close the `block }`.

6. Print `outer_variable` again.

[🚀 Run this code in Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=fn+main%28%29+%7B%0D%0A++++let+outer_variable+%3D+100%3B%0D%0A%0D%0A++++%7B%0D%0A++++++++let+inner_variable+%3D+200%3B%0D%0A++++++++println%21%28%22Value+inside+block-+outer_variable%3A+%7B%7D%22%2C+outer_variable%29%3B%0D%0A++++++++println%21%28%22inner_variable%3A+%7B%7D%22%2C+inner_variable%29%3B%0D%0A++++%7D%0D%0A%0D%0A++++println%21%28%22Value+outside+block-+outer_variable%3A+%7B%7D%22%2C+outer_variable%29%3B%0D%0A++++%2F*println%21%28%22Value+outside+block-+outer_variable%3A+%7B%7D%22%2C+inside_variable%29%3B%0D%0A+++++++++++++++++++++++++++++++++++++++++++++++++++++++++%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E%5E+not+found+in+this+scope%0D%0A++++*%2F%0D%0A%7D)
