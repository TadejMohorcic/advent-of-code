use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use regex::Regex;

pub fn main() -> Result<(), Error> {
    // let path = "input/day03-test.txt";
    let path = "input/day03.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut string_input = String::new();

    for line in buffered.lines() {
        let line_ok = line?;
        string_input.push_str(&line_ok);
    }

    let part_one = scan_for_multiplicators(
        &string_input,
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(),
    );
    let part_two = scan_for_multiplicators(
        &string_input,
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap(),
    );

    println!("--- Day 3: Mull It Over ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn scan_for_multiplicators(string: &String, re: Regex) -> u64 {
    let mut result = 0;
    let mut multiply = true;

    for capture in re.captures_iter(string) {
        if let Some(x) = capture.get(1) {
            let y = capture.get(2).unwrap();

            if multiply {
                let x_int: u64 = x.as_str().parse().unwrap();
                let y_int: u64 = y.as_str().parse().unwrap();

                result += x_int * y_int;
            }
        } else {
            let instruction = &capture[0];
            multiply = if instruction == "don't()" {
                false
            } else {
                true
            };
        }
    }

    result
}
