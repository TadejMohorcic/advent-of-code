use std::path::Path;

pub fn main() {
    let batteries = parse_input("input/day03-test.txt");
    let capacity_one = 2;
    let capacity_two = 12;
    let part_one = highest_joltage(&batteries, capacity_one);
    let part_two = highest_joltage(&batteries, capacity_two);

    println!("--- Day 3: Lobby ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<u64>> {
    let mut batteries = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let battery: Vec<u64> = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            batteries.push(battery);
        }
    }

    batteries
}

fn highest_joltage(batteries: &[Vec<u64>], capacity: usize) -> u64 {
    let mut total_joltage = 0;

    for battery in batteries {
        let mut battery_acc = Vec::new();
        let length = battery.len();

        for (i, bat) in battery.iter().enumerate() {
            let remaining_bats = length - i - 1;

            while !battery_acc.is_empty()
                && battery_acc[battery_acc.len() - 1] < bat
                && remaining_bats >= capacity - battery_acc.len()
            {
                battery_acc.pop();
            }

            if battery_acc.len() < capacity {
                battery_acc.push(bat);
            }
        }

        total_joltage += battery_acc.iter().fold(0, |acc, x| acc * 10 + *x);
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let batteries = parse_input("input/day03-test.txt");
        assert_eq!(highest_joltage(&batteries, 2), 357);
    }

    #[test]
    fn part_two_example() {
        let batteries = parse_input("input/day03-test.txt");
        assert_eq!(highest_joltage(&batteries, 12), 3121910778619)
    }
}
