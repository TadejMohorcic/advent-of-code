use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() -> Result<(), Error> {
    // let path = "input/day22-test.txt";
    let path = "input/day22.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut secret_numbers = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let num: i64 = line_ok.trim().parse().expect("Should be a number!");
        secret_numbers.push(num);
    }

    let (part_one, part_two) = get_new_secret_numbers(secret_numbers);

    println!("--- Day 22: Monkey Market ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn next_secret_number(mut num: i64) -> i64 {
    let mut mix = num * 64;
    num = (num ^ mix) % 16777216;
    mix = num / 32;
    num = (num ^ mix) % 16777216;
    mix = num * 2048;
    num = (num ^ mix) % 16777216;

    num
}

fn get_new_secret_numbers(nums: Vec<i64>) -> (i64, i64) {
    let mut total_cost = 0;

    let mut bananas: HashMap<VecDeque<i64>, i64> = HashMap::new();

    for mut num in nums {
        let mut sequence = VecDeque::new();
        let mut seen_sequences = HashSet::new();

        for _ in 0..2000 {
            let new_num = next_secret_number(num);

            sequence.push_back(new_num % 10 - num % 10);

            if sequence.len() > 4 {
                sequence.pop_front();
            }

            if sequence.len() == 4 && !seen_sequences.contains(&sequence) {
                *bananas.entry(sequence.clone()).or_insert(0) += new_num % 10;
                seen_sequences.insert(sequence.clone());
            }

            num = new_num;
        }
        total_cost += num;
    }

    (total_cost, bananas.iter().map(|(_, v)| *v).max().unwrap())
}
