use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day03-test.txt";
    let path = "input/day03.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut batteries = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let digits: Vec<u64> = line_ok.trim().chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
        batteries.push(digits);
    }

    let part_one = highest_joltage(&batteries, 2);
    let part_two = highest_joltage(&batteries, 12);

    println!("--- Day 3: Lobby ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn highest_joltage(batteries: &[Vec<u64>], capacity: usize) -> u64 {
    let mut total_joltage = 0;

    for battery in batteries {
        let mut battery_acc = Vec::new();
        let length = battery.len();

        for (i, bat) in battery.iter().enumerate() {
            let remaining_bats = length - i - 1;

            while !battery_acc.is_empty() && battery_acc[battery_acc.len() - 1] < bat && remaining_bats >= capacity - battery_acc.len() {
                battery_acc.pop();
            }

            if battery_acc.len() < capacity {
                battery_acc.push(bat);
            }
        }

        total_joltage += battery_acc.iter().fold(0, |acc, x| acc * 10 + *x);
    }

    total_joltage
}