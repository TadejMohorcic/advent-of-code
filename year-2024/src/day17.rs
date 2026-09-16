use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (mut register, program) = parse_input("input/day17.txt")?;
    let part_one = run_program(&mut register, &program);
    let part_two = find_starting_value(&program, 0, 0);

    println!("--- Day 17: Chronospatial Computer ---");
    println!(" - Part one solution: {:?}", part_one);
    println!(" - Part two solution: {}\n", part_two.iter().min().unwrap());

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<([i64; 3], Vec<i64>)> {
    let lines = crate::read_lines(&filename)?;
    let mut register = [0; 3];
    let mut register_counter = 0;
    let mut program = None;

    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }

        if register_counter < 3 {
            register[register_counter] = line
                .trim()
                .split(": ")
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            register_counter += 1;
        } else {
            let p = line.trim().split(": ").last().unwrap();
            program = Some(p.split(',').map(|n| n.parse::<i64>().unwrap()).collect());
        }
    }

    Ok((register, program.unwrap()))
}

fn run_program(register: &mut [i64; 3], program: &[i64]) -> String {
    let mut pointer = 0;
    let n = program.len();
    let mut output: Vec<i64> = Vec::new();

    loop {
        if pointer >= n {
            return output
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }

        let mut jump = false;
        let instruction = program[pointer];
        let literal = program[pointer + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => register[0],
            5 => register[1],
            6 => register[2],
            _ => 7,
        };
        match instruction {
            0 => register[0] /= 2_i64.pow(combo as u32),
            1 => {
                let b = register[1];
                register[1] = b ^ literal;
            }
            2 => register[1] = combo % 8,
            3 => {
                if register[0] != 0 {
                    pointer = literal as usize;
                    jump = true;
                }
            }
            4 => {
                let b = register[1];
                let c = register[2];
                register[1] = b ^ c;
            }
            5 => output.push(combo % 8),
            6 => register[1] = register[0] / 2_i64.pow(combo as u32),
            _ => register[2] = register[0] / 2_i64.pow(combo as u32),
        }

        if !jump {
            pointer += 2;
        }
    }
}

// Reddit solution
fn find_starting_value(program: &[i64], mut a: i64, index: usize) -> Vec<i64> {
    let n = program.len();

    if index == n {
        return vec![a];
    }

    a <<= 3;
    let mut solutions = Vec::new();

    for i in 0..8 {
        let new_a = a + i;
        let mut b = (new_a % 8) ^ 3;
        b = (b ^ (new_a >> b) ^ 5) % 8;

        if b == program[n - 1 - index] {
            solutions.extend(find_starting_value(program, new_a, index + 1));
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (mut register, program) = parse_input("input/day17-test.txt").unwrap();
        assert_eq!(run_program(&mut register, &program), "4,6,3,5,6,3,5,2,1,0");
    }
}
