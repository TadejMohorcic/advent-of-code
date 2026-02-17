use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day06_test.txt";
    let path = "input/day06.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut operations = Vec::new();
    let mut horizontal_numbers = Vec::new();
    let mut vertical_numbers = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let number_line: Vec<u64> = line_ok.trim().split_whitespace().filter_map(|x| x.parse().ok()).collect();
        let number_stack: Vec<u64> = line_ok.chars().map(|x| match x {'1'..='9' => x.to_digit(10).unwrap() as u64, _ => 0}).collect();
        
        if number_line.is_empty() {
            operations = line_ok.trim().chars().filter(|x| *x != ' ').collect();
        }
        else {
            horizontal_numbers.push(number_line);
        }

        if vertical_numbers.is_empty() {
            vertical_numbers = number_stack;
        }
        else {
            vertical_numbers = vertical_numbers.iter().zip(number_stack.iter()).map(|(a, b)| if *b == 0 {*a} else {10 * a + b}).collect();
        }
    }

    let part_one = calculate_top_down(horizontal_numbers, &operations);
    let part_two = calculate_left_right(vertical_numbers, &operations);

    println!("--- Day 6: Trash Compactor ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn calculate_top_down(numbers: Vec<Vec<u64>>, instructions: &Vec<char>) -> u64 {
    let mut top_down_sum: Vec<(usize, u64)> = numbers[0].iter().enumerate().map(|(x, y)| (x, *y)).collect();
    let n = numbers.len();

    for i in 1..n {
        top_down_sum = top_down_sum.iter().zip(numbers[i].iter()).map(|((j, a), b)| if instructions[*j] == '*' {(*j, a * b)} else {(*j, a + b)}).collect();
    }

    top_down_sum.iter().fold(0, |acc, (_, x)| acc + x)
}

fn calculate_left_right(numbers: Vec<u64>, instructions: &Vec<char>) -> u64 {
    let mut total_value = 0;

    let mut index = 0;
    let mut current_value = if instructions[index] == '*' {1} else {0};

    for n in &numbers {
        if *n == 0 {
            total_value += current_value;
            index += 1;
            current_value = if instructions[index] == '*' {1} else {0};
        }
        else {
            current_value = if instructions[index] == '*' {current_value * n} else {current_value + n};
        }
    }

    total_value += current_value;

    total_value
}

