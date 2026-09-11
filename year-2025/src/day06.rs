use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (hn, vn, ops) = parse_input("input/day06.txt")?;
    let part_one = calculate_horizontal(&hn, &ops);
    let part_two = calculate_vertical(&vn, &ops);

    println!("--- Day 6: Trash Compactor ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

type MathProblem = (Vec<Vec<i64>>, Vec<i64>, Vec<char>);

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<MathProblem> {
    let lines = crate::read_lines(filename)?;
    let mut operations = Vec::new();
    let mut horizontal_numbers = Vec::new();
    let mut vertical_numbers = Vec::new();

    for line in lines.map_while(Result::ok) {
        let number_line: Vec<i64> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let mut number_stack: Vec<i64> = line
            .chars()
            .map(|x| match x {
                '1'..='9' => x.to_digit(10).unwrap() as i64,
                _ => 0,
            })
            .collect();

        if number_line.is_empty() {
            operations = line
                .split_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect();
        } else {
            horizontal_numbers.push(number_line);
        }

        if vertical_numbers.is_empty() {
            vertical_numbers = number_stack;
        } else {
            let desired_len = number_stack.len().max(vertical_numbers.len());

            number_stack.resize(desired_len, 0);
            vertical_numbers.resize(desired_len, 0);

            vertical_numbers = vertical_numbers
                .iter()
                .zip(number_stack.iter())
                .map(|(a, b)| if *b == 0 { *a } else { 10 * a + b })
                .collect();

            if vertical_numbers.len() != number_stack.len() {
                vertical_numbers.extend(&number_stack[vertical_numbers.len()..])
            }
        }
    }

    Ok((horizontal_numbers, vertical_numbers, operations))
}

fn calculate_horizontal(numbers: &[Vec<i64>], operations: &[char]) -> i64 {
    let mut horizontal_operations = numbers[0].clone();
    let n = numbers.len();

    for number_line in numbers.iter().take(n).skip(1) {
        for (j, num) in number_line.iter().enumerate() {
            if operations[j] == '*' {
                horizontal_operations[j] *= num;
            } else {
                horizontal_operations[j] += num;
            }
        }
    }

    horizontal_operations.iter().sum()
}

fn calculate_vertical(numbers: &[i64], operations: &[char]) -> i64 {
    let vertical_operations: Vec<i64> = numbers
        .split(|&n| n == 0)
        .zip(operations.iter())
        .map(|(nums, op)| {
            nums.iter().fold(if *op == '*' { 1 } else { 0 }, |acc, x| {
                if *op == '*' { acc * x } else { acc + x }
            })
        })
        .collect();

    vertical_operations.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (hn, _, ops) = parse_input("input/day06-test.txt").unwrap();
        assert_eq!(calculate_horizontal(&hn, &ops), 4277556);
    }

    #[test]
    fn part_two_example() {
        let (_, vn, ops) = parse_input("input/day06-test.txt").unwrap();
        assert_eq!(calculate_vertical(&vn, &ops), 3263827);
    }
}
