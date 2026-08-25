use std::collections::HashMap;
use std::path::Path;

pub fn main() {
    let (left, right) = parse_input("input/day01.txt");
    let part_one = get_distance(&left, &right);
    let part_two = get_similarity_score(&left, &right);

    println!("\n--- Day 1: Historian Hysteria ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> (Vec<i64>, Vec<i64>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            left_list.push(numbers.next().unwrap());
            right_list.push(numbers.next().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn get_distance(left_list: &[i64], right_list: &[i64]) -> u64 {
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

fn get_similarity_score(left_list: &[i64], right_list: &[i64]) -> i64 {
    let mut similarity_score = 0;
    let mut cache = HashMap::new();

    for number in left_list {
        if let Some(occurance) = cache.get(number) {
            similarity_score += number * occurance;
        } else {
            let occurance = right_list
                .iter()
                .filter(|x| *x == number)
                .fold(0, |acc, _| acc + 1);
            cache.insert(*number, occurance);

            similarity_score += number * occurance;
        }
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (left, right) = parse_input("input/day01-test.txt");
        assert_eq!(get_distance(&left, &right), 11);
    }

    #[test]
    fn part_two_example() {
        let (left, right) = parse_input("input/day01-test.txt");
        assert_eq!(get_similarity_score(&left, &right), 31);
    }
}
