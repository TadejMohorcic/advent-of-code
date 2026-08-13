use std::collections::HashSet;
use std::ops::Range;
use std::path::Path;

pub fn main() {
    let ranges = parse_input("input/day02-test.txt");
    let longest_num = ranges.iter().map(|r| number_len(r.end)).max().unwrap_or(0);
    let part_one = invalid_id_sum(&ranges, false, longest_num);
    let part_two = invalid_id_sum(&ranges, true, longest_num);

    println!("--- Day 2: Gift Shop ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Range<u64>> {
    let mut ranges = Vec::new();
    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            ranges.extend(line.trim().split(',').map(|part| {
                let (start_str, end_str) = part.split_once('-').unwrap();
                let start = start_str.parse::<u64>().unwrap();
                let end = end_str.parse::<u64>().unwrap() + 1;
                start..end
            }));
        }
    }

    ranges
}

fn number_len(n: u64) -> usize {
    if n == 0 { 0 } else { (n.ilog10() + 1) as usize }
}

fn generators(n: usize, more_then_two: bool, longest_num: usize) -> Vec<Vec<u64>> {
    let mut generators = Vec::new();
    let mut generator = 10;

    for i in 1..=n {
        let mut generators_i = Vec::new();
        let mut new_generator = generator;

        while number_len(new_generator) < longest_num {
            new_generator += 1;
            generators_i.push(new_generator);

            if !more_then_two {
                break;
            }

            new_generator *= 10_u64.pow(i as u32);
        }

        generators.push(generators_i);
        generator *= 10;
    }

    generators
}

fn invalid_id_sum(ranges: &[Range<u64>], more_then_two: bool, longest_num: usize) -> u64 {
    let size_to_check = longest_num / 2;
    let max_number = 10u64.pow(size_to_check as u32) - 1;
    let generators = generators(size_to_check, more_then_two, longest_num);
    let mut invalid_ids = HashSet::new();

    for i in 1..=max_number {
        let valid_generators = &generators[number_len(i) - 1];

        for generator in valid_generators {
            let num = i * generator;

            if ranges.iter().any(|r| r.contains(&num)) {
                invalid_ids.insert(num);
            }
        }
    }

    invalid_ids.iter().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let ranges = parse_input("input/day02-test.txt");
        let longest_num = ranges.iter().map(|r| number_len(r.end)).max().unwrap_or(0);
        assert_eq!(invalid_id_sum(&ranges, false, longest_num), 1227775554);
    }

    #[test]
    fn part_two_example() {
        let ranges = parse_input("input/day02-test.txt");
        let longest_num = ranges.iter().map(|r| number_len(r.end)).max().unwrap_or(0);
        assert_eq!(invalid_id_sum(&ranges, true, longest_num), 4174379265);
    }
}
