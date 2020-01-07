# Advent of Code 2019

My rust solutions. Opted not to use cargo-aoc for now. To try different days just edit `main.rs` to call aocDD::solve_first(input)/solve_second and provide file path as argument when running.

For example, `cargo run -- src/aoc10/input`

Each day folder has a `mod.rs` file which has `solve_first(input: &str)` and `solve_second(input: &str)` for the day's exercise.
Main.rs calls and prints the result.

To run all tests:

`cargo test`

To test a specific day:

`cargo test aocDD`
