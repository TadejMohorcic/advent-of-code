use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::HashMap;

pub fn main() -> Result<(), Error> {
    // let path = "input/day01-test.txt";
    let path = "input/day01.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let numbers: Vec<u64> = line_ok.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        list_a.push(numbers[0]);
        list_b.push(numbers[1]);
    }

    list_a.sort();
    list_b.sort();

    let part_one = calculate_distance(&list_a, &list_b);
    let part_two = get_similarity_score(&list_a, &list_b);

    println!("");
    println!("--- Day 1: Historian Hysteria ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn calculate_distance(l1: &Vec<u64>, l2: &Vec<u64>) -> u64 {
    let distance = l1.iter().zip(l2.iter()).map(|(x, y)| x.abs_diff(*y)).fold(0, |acc, x| acc + x);

    distance
}

fn get_similarity_score(l1: &Vec<u64>, l2: &Vec<u64>) -> u64 {
    let mut similarity_score = 0;

    let mut cache = HashMap::new();

    for number in l1 {
        if let Some(occurance) = cache.get(number) {
            similarity_score += number * occurance;
        }
        else {
            let occurance = l2.iter().filter(|x| *x == number).fold(0, |acc, _| acc + 1);
            cache.insert(*number, occurance);

            similarity_score += number * occurance;
        }
    }

    similarity_score
}