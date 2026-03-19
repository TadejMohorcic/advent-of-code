use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashMap;

pub fn main() -> Result<(), Error> {
    // let path = "input/day19-test.txt";
    let path = "input/day19.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut is_towel = false;

    let mut patterns: Option<Vec<String>> = None;
    let mut towels = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        if line_ok.is_empty() {
            is_towel = true;
            continue;
        }

        if is_towel {
            towels.push(line_ok.trim().to_string());
        } else {
            patterns = Some(line_ok.trim().split(", ").map(|x| x.to_string()).collect());
        }
    }

    let patterns = patterns.unwrap();

    let (part_one, part_two) = check_designs(&towels, &patterns);

    println!("--- Day 19: Linen Layout ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn check_design(
    towel: &String,
    patterns: &Vec<String>,
    index: usize,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    let n = towel.len();

    if index == n {
        return 1;
    } else {
        if let Some(res) = cache.get(&towel[index..].to_string()) {
            return *res;
        } else {
            let mut result = 0;

            for pattern in patterns {
                let m = pattern.len();

                if index + m <= n && &towel[index..index + m] == pattern {
                    result += check_design(towel, patterns, index + m, cache);
                }
            }

            cache.insert(towel[index..].to_string(), result);

            return result;
        }
    }
}

fn check_designs(towels: &Vec<String>, patterns: &Vec<String>) -> (u64, u64) {
    let mut part_one = 0;
    let mut part_two = 0;

    let mut cache = HashMap::new();

    for towel in towels {
        let designs_avalible = check_design(towel, patterns, 0, &mut cache);

        part_one += (designs_avalible > 0) as u64;
        part_two += designs_avalible;
    }

    (part_one, part_two)
}
