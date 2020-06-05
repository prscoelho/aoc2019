# Advent of Code 2019

My rust solutions for [Advent of Code 2019.](https://adventofcode.com/2019)

## Goals of this project

My goal for this repository is to have a readable, maintainable and performant solution for every puzzle.
Hopefully I can accomplish it, and I'd love any input on where things could be better.

## Running

To try different days just edit `main.rs` to call aocDD::solve_first(input)/solve_second and provide file path as argument when running.

For example, `cargo run --release -- src/aoc10/input`

Each day folder has a `mod.rs` file which has `solve_first(input: &str)` and `solve_second(input: &str)` for the day's exercise.
Main.rs calls and prints the result.

## Testing

To run all tests (limiting threads as there's too many tests to run):

`cargo test -- --test-threads=1`

To test a specific day:

`cargo test aocDD`
