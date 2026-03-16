use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    let path = "input/day01.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut instructions = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let mut chars = line_ok.trim().chars();

        let direction: i32;

        match chars.next().unwrap() {
            'L' => direction = -1,
            'R' => direction = 1,
            _ => unreachable!("Invalid direction!"),
        }

        let rotation: i32 = chars
            .as_str()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        instructions.push(rotation * direction);
    }

    let part_one = count_zeros(&instructions, 50);
    let part_two = passing_zero(&instructions, 50);

    println!("");
    println!("--- Day 1: Secret Entrance ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn count_zeros(instructions: &[i32], mut position: i32) -> i32 {
    let mut result = 0;

    for instruction in instructions {
        position = (position + instruction).rem_euclid(100);

        if position == 0 {
            result += 1;
        }
    }

    result
}

fn passing_zero(instructions: &[i32], mut position: i32) -> i32 {
    let mut result = 0;

    for instruction in instructions {
        let old_position = position;
        let mut current_rotations = 0;

        position = (position + instruction).rem_euclid(100);
        current_rotations += instruction.abs() / 100;

        if *instruction >= 0 {
            current_rotations += (position < old_position) as i32;
        } else {
            current_rotations +=
                (old_position != 0 && (position > old_position || position == 0)) as i32;
        }

        result += current_rotations;
    }

    result
}
