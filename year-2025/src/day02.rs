use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;
use std::ops::Range;

pub fn main() -> Result<(), Error> {
    let path = "input/day02.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut ranges = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let line_split: Vec<&str> = line_ok.trim().split(',').collect();

        for line_el in line_split {
            let r: Vec<u64> = line_el.split('-').map(|x| x.parse().unwrap()).collect();
            ranges.push(r[0]..r[1]);
        }
    }

    let mask = generate_mask(5, true);
    let mask_repeats = generate_mask(5, false);

    let part_one = sum_valid_ids(&ranges, &mask);
    let part_two = sum_valid_ids(&ranges, &mask_repeats);

    println!("--- Day 2: Gift Shop ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn get_number_length(mut n: u64) -> usize {
    let mut length = 0;

    while n > 0 {
        let d = n % 10;
        n = (n - d) / 10;
        length += 1;
    }

    length
}

fn generate_mask(n: usize, two_repeats: bool) -> Vec<Vec<u64>> {
    let mut masks = Vec::new();
    let mut generator = 10;

    for i in 1..=n {
        let mut current_masks = Vec::new();
        let mut new_generator = generator;

        while get_number_length(new_generator) < 10 {
            new_generator += 1;
            current_masks.push(new_generator);
            new_generator *= 10_u64.pow(i as u32);

            if two_repeats {
                break;
            }
        }

        masks.push(current_masks);

        generator *= 10;
    }

    masks
}

fn sum_valid_ids(ranges: &[Range<u64>], mask: &[Vec<u64>]) -> u64 {
    let mut valid_ids: HashSet<u64> = HashSet::new();

    for i in 1..99_999 {
        let correct_mask = &mask[get_number_length(i) - 1];

        for generator in correct_mask {
            let number = generator * i;

            for range in ranges {
                if range.contains(&number) {
                    valid_ids.insert(number);
                }
            }
        }
    }

    let valid_id_sum = valid_ids.iter().fold(0, |acc, x| acc + x);

    valid_id_sum
}
