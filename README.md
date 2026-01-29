# Rust Learning Workspace

This repository contains a collection of small Rust programs created as part of a daily learning exercise. The goal is to get hands-on practice with the Rust programming language by exploring its features through simple, focused examples.

## Project Structure

The workspace is organized by day, with each `src/days/day<N>` directory representing a new set of lessons or concepts.

The `Cargo.toml` file is set up to treat each example `.rs` file as a separate executable binary. This allows each program to be run independently.

## Building and Running

You can build and run these programs using standard Cargo commands.

### Building All Programs

To compile all the example programs, run the following command from the root of the project:

```bash
cargo build
```

### Running a Specific Program

Because the project contains multiple binaries, you must specify which program you want to run using the `--bin` flag. The binary name corresponds to the filename (without the `.rs` extension).

For example:

*   **To run `greetings.rs` from day 1:**
    ```bash
    cargo run --bin greetings
    ```

*   **To run `scope.rs` from day 2:**
    ```bash
    cargo run --bin scope
    ```
*   **To run `shadowing.rs` from day 3:**
    ```bash
    cargo run --bin shadowing
    ```

Feel free to explore the code and experiment!
