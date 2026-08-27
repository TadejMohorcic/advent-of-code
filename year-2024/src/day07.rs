use std::path::Path;

pub fn main() {
    let equations = parse_input("input/day07.txt");
    let part_one = calibrate_equations(&equations, false);
    let part_two = calibrate_equations(&equations, true);

    println!("--- Day 7: Bridge Repair ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<(i64, Vec<i64>)> {
    let mut equations = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let split = line.trim().split_once(": ").unwrap();
            let target = split.0.parse::<i64>().unwrap();
            let numbers = split
                .1
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            equations.push((target, numbers))
        }
    }

    equations
}

fn can_produce_target(
    target: i64,
    numbers: &[i64],
    current_value: i64,
    index: usize,
    part: bool,
) -> bool {
    if index == numbers.len() {
        return current_value == target;
    } else if current_value > target {
        return false;
    }

    let addition = can_produce_target(
        target,
        numbers,
        current_value + numbers[index],
        index + 1,
        part,
    );
    let multiplication = can_produce_target(
        target,
        numbers,
        current_value * numbers[index],
        index + 1,
        part,
    );
    let num_len = numbers[index].checked_ilog10().unwrap_or(0) + 1;
    let concatenation = part
        && can_produce_target(
            target,
            numbers,
            current_value * 10_i64.pow(num_len) + numbers[index],
            index + 1,
            part,
        );

    addition || multiplication || concatenation
}

fn calibrate_equations(equations: &[(i64, Vec<i64>)], part: bool) -> i64 {
    equations
        .iter()
        .filter_map(|(target, numbers)| {
            can_produce_target(*target, numbers, numbers[0], 1, part).then_some(target)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let equations = parse_input("input/day07-test.txt");
        assert_eq!(calibrate_equations(&equations, false), 3749);
    }

    #[test]
    fn part_two_example() {
        let equations = parse_input("input/day07-test.txt");
        assert_eq!(calibrate_equations(&equations, true), 11387);
    }
}
