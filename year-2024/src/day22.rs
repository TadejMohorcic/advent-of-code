use std::collections::{HashMap, HashSet};
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let secret_numbers = parse_input("input/day22.txt")?;
    let (part_one, part_two) = get_new_secret_numbers(&secret_numbers);

    println!("--- Day 22: Monkey Market ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<i64>> {
    let lines = crate::read_lines(&filename)?;
    let mut secret_numbers = Vec::new();

    for line in lines.map_while(Result::ok) {
        secret_numbers.push(line.trim().parse::<i64>().unwrap());
    }

    Ok(secret_numbers)
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

fn get_new_secret_numbers(numbers: &[i64]) -> (i64, i64) {
    let mut total_cost = 0;
    let mut bananas = HashMap::new();

    for n in numbers {
        let mut secret = *n;
        let mut sequence = [0; 4];
        let mut seen = HashSet::new();

        for step in 0..2000 {
            let next = next_secret_number(secret);
            let price = next % 10;
            sequence.rotate_left(1);
            sequence[3] = price - secret % 10;

            if step > 2 && seen.insert(sequence) {
                *bananas.entry(sequence).or_insert(0) += price;
            }

            secret = next;
        }

        total_cost += secret;
    }

    (total_cost, bananas.values().copied().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let secret_numbers = parse_input("input/day22-test1.txt").unwrap();
        let (part_one, _) = get_new_secret_numbers(&secret_numbers);
        assert_eq!(part_one, 37327623);
    }

    #[test]
    fn part_two_example() {
        let secret_numbers = parse_input("input/day22-test2.txt").unwrap();
        let (_, part_two) = get_new_secret_numbers(&secret_numbers);
        assert_eq!(part_two, 23);
    }
}
