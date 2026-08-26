use regex::Regex;
use std::path::Path;

pub fn main() {
    let memory = parse_input("input/day03.txt");
    let instructions1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let instructions2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let part_one = multiply_instructions(&memory, instructions1);
    let part_two = multiply_instructions(&memory, instructions2);

    println!("--- Day 3: Mull It Over ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> String {
    let mut memory = String::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            memory.push_str(line.trim());
        }
    }

    memory
}

fn multiply_instructions(string: &str, re: Regex) -> i64 {
    let mut result = 0;
    let mut multiply = true;

    for capture in re.captures_iter(string) {
        match capture.get(1) {
            Some(x) => {
                if multiply {
                    let x = x.as_str().parse::<i64>().unwrap();
                    let y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();

                    result += x * y;
                }
            }
            None => multiply = &capture[0] != "don't()",
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let memory = parse_input("input/day03-test1.txt");
        let instructions = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        assert_eq!(multiply_instructions(&memory, instructions), 161);
    }

    #[test]
    fn part_two_example() {
        let memory = parse_input("input/day03-test2.txt");
        let instructions = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        assert_eq!(multiply_instructions(&memory, instructions), 48);
    }
}
