# Solutions to advent of code

This repository contains my solutions to problems at [Advent of Code](https://adventofcode.com/).

## Requirements
 
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, via `cargo`)

## Repository structure

Code is organized in folders `year-<year>` by year. Each year contains:
 - `src/main.rs` - entry point that runs all days
 - `src/day<day>.rs` — solution code for each day

Example:
```
year-2025/
├── src/
│   ├── main.rs
│   ├── day01.rs
│   ├── day02.rs
│   └── ...
└── input/
    ├── day01.txt
    └── day02.txt
```

> **Note:** Since AoC asks that puzzle inputs are not redistributed, you'll need to supply your own (see below).

## Running against your own input
 
To test the code against your own puzzle inputs:
 
1. Create an `input` folder inside the year you want to run.
2. Add your input as `day<day>.txt`.
3. Run:

```
cd year-<year>
cargo run
```

This prints solutions for all days with inputs present, in the following format:
 
```
--- Day <day>: <title> ---
 - Part one: <part_one_solution>
 - Part two: <part_two_solution>
```

## Running the example tests
 
To run tests against the example inputs provided in each puzzle:
 
1. Copy the example into `input/day<day>-test.txt`.
   - If part one and two use different examples, use `input/day<day>-test1.txt` and `input/day<day>-test2.txt` instead.
2. Run:
```
cargo test
```

## TODO
- year-2024 is still in progress
- add solutions to previous years
