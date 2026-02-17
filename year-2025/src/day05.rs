use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::ops::Range;
use std::cmp::max;

pub fn main() -> Result<(), Error> {
    // let path = "input/day05_test.txt";
    let path = "input/day05.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let parsed_line: Vec<u64> = line_ok.trim().split('-').filter_map(|x| x.parse().ok()).collect();

        match parsed_line.len() {
            2 => ranges.push(parsed_line[0]..parsed_line[1]+1),
            1 => ingredients.push(parsed_line[0]),
            _ => continue
        }
    }

    let merged_ranges = merge_ranges(ranges);

    let part_one = count_fresh_ingredients(&ingredients, &merged_ranges);
    let part_two = all_fresh_ingredients(&merged_ranges);

    println!("--- Day 5: Cafeteria ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged_ranges: Vec<Range<u64>> = Vec::new();

    for range in ranges {
        let previous_range = merged_ranges.pop();
        match previous_range {
            Some(r) => {
                if range.start <= r.end {
                    merged_ranges.push(r.start..max(r.end, range.end));
                }
                else {
                    merged_ranges.push(r);
                    merged_ranges.push(range);
                }
            }
            None => merged_ranges.push(range)
        }
    }

    merged_ranges
}

fn count_fresh_ingredients(ingredients: &Vec<u64>, ranges: &Vec<Range<u64>>) -> usize {
    let mut fresh_ingredients = 0;

    for ingredient in ingredients {
        for range in ranges {
            if range.contains(ingredient) {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    fresh_ingredients
}

fn all_fresh_ingredients(ranges: &Vec<Range<u64>>) -> usize {
    let mut fresh_ingredients = 0;

    for range in ranges {
        let current_fresh = range.end - range.start;
        fresh_ingredients += current_fresh as usize;
    }

    fresh_ingredients
}