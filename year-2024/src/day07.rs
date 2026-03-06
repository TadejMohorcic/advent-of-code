use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day07-test.txt";
    let path = "input/day07.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut equations = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let (t, nums) = line_ok.trim().split_once(": ").unwrap();
        let target: i64 = t.parse().unwrap();
        let numbers: Vec<i64> = nums
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let eq = Equation {
            target: target,
            numbers: numbers,
        };
        equations.push(eq);
    }

    let part_one = calculate_calibration(&equations, false);
    let part_two = calculate_calibration(&equations, true);

    println!("--- Day 7: Bridge Repair ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

fn calculate_calibration(equations: &Vec<Equation>, part: bool) -> i64 {
    equations
        .iter()
        .map(|equation| {
            let starting_value = equation.numbers[0];
            if apply_operations(equation, starting_value, 1, part) {
                equation.target
            } else {
                0
            }
        })
        .sum()
}

fn apply_operations(
    equation: &Equation,
    current_value: i64,
    current_index: usize,
    part: bool,
) -> bool {
    if current_index == equation.numbers.len() {
        return current_value == equation.target;
    } else if current_value > equation.target {
        return false;
    } else {
        let next_number = equation.numbers[current_index];
        let n = next_number.checked_ilog10().unwrap_or(0) + 1;

        let b1 = apply_operations(
            equation,
            current_value + next_number,
            current_index + 1,
            part,
        );
        let b2 = apply_operations(
            equation,
            current_value * next_number,
            current_index + 1,
            part,
        );
        let b3 = if part {
            apply_operations(
                equation,
                current_value * 10_i64.pow(n) + next_number,
                current_index + 1,
                part,
            )
        } else {
            false
        };

        return b1 || b2 || b3;
    }
}
