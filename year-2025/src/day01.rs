use std::path::Path;

pub fn main() {
    let instructions = parse_input("input/day01.txt");
    let starting_pos = 50;
    let part_one = land_on_zero(&instructions, starting_pos);
    let part_two = pass_zero(&instructions, starting_pos);

    println!("");
    println!("--- Day 1: Secret Entrance ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<i64> {
    let mut instructions = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut line = line.trim().chars();
            let direction = if line.next().unwrap() == 'L' { -1 } else { 1 };
            let rotation = line
                .by_ref()
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, c| 10 * acc + c.to_digit(10).unwrap() as i64);
            instructions.push(direction * rotation);
        }
    }

    instructions
}

fn land_on_zero(instructions: &[i64], mut position: i64) -> i64 {
    let mut result = 0;

    for instruction in instructions {
        position = (position + instruction).rem_euclid(100);
        result += (position == 0) as i64;
    }

    result
}

fn pass_zero(instructions: &[i64], mut position: i64) -> i64 {
    let mut result = 0;

    for instruction in instructions {
        let old_position = position;
        position = (position + instruction).rem_euclid(100);
        let mut current_rotations = instruction.abs() / 100;

        if *instruction >= 0 {
            current_rotations += (position < old_position) as i64;
        } else {
            current_rotations +=
                (old_position != 0 && (position > old_position || position == 0)) as i64;
        }

        result += current_rotations;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let instructions = parse_input("input/day01-test.txt");
        assert_eq!(land_on_zero(&instructions, 50), 3);
    }

    #[test]
    fn part_two_example() {
        let instructions = parse_input("input/day01-test.txt");
        assert_eq!(pass_zero(&instructions, 50), 6);
    }
}
