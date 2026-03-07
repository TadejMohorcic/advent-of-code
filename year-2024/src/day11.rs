use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashMap;

pub fn main() -> Result<(), Error> {
    // let path = "input/day11-test.txt";
    let path = "input/day11.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut stones: Option<HashMap<u64, usize>> = None;

    for line in buffered.lines() {
        let line_ok = line?;

        stones = Some(
            line_ok
                .trim()
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), 1))
                .collect(),
        )
    }

    let stones = stones.unwrap();

    let part_one = blink(stones.clone(), 25);
    let part_two = blink(stones, 75);

    println!("--- Day 11: Plutonian Pebbles ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn do_a_blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones {
        match stone {
            0 => *new_stones.entry(1).or_insert(0) += count,
            x if (x.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                let modulo = 10_u64.pow((x.checked_ilog10().unwrap_or(0) + 1) / 2);

                let right_num = x % modulo;
                let left_num = x / modulo;

                *new_stones.entry(left_num).or_insert(0) += count;
                *new_stones.entry(right_num).or_insert(0) += count;
            }
            _ => *new_stones.entry(stone * 2024).or_insert(0) += count,
        }
    }

    new_stones
}

fn blink(mut stones: HashMap<u64, usize>, n: usize) -> usize {
    for _ in 0..n {
        stones = do_a_blink(stones);
    }

    stones.iter().fold(0, |acc, (_, c)| acc + c)
}
