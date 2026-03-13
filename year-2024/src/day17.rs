use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day17-test.txt";
    let path = "input/day17.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut index = 0;
    let mut register = [0; 3];

    let mut instructions: Option<Vec<usize>> = None;

    for line in buffered.lines() {
        let line_ok = line?;

        if index > 2 {
            if !line_ok.is_empty() {
                instructions = Some(
                    line_ok
                        .trim()
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split(',')
                        .map(|x| x.parse().expect("Should be a number!"))
                        .collect(),
                );
            }
        } else {
            let value: i64 = line_ok
                .trim()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .expect("Should be a number!");

            register[index] = value;
            index += 1;
        }
    }

    let instructions = instructions.unwrap();

    let part_one = run_program(&mut register, &instructions);
    let part_two = find_starting_value(0, &instructions, 0);

    println!("--- Day 17: Chronospatial Computer ---");
    println!(" - Part one solution: {:?}", part_one);
    println!(" - Part two solution: {}", part_two.iter().min().unwrap());
    println!("");

    Ok(())
}

fn run_program(register: &mut [i64; 3], instructions: &Vec<usize>) -> Vec<usize> {
    let mut pointer = 0;
    let n = instructions.len();

    let mut out = Vec::new();

    loop {
        if pointer >= n {
            return out;
        }

        let instruction = instructions[pointer];
        let literal = instructions[pointer + 1] as i64;
        let combo = match literal {
            0..=3 => literal,
            4 => register[0],
            5 => register[1],
            6 => register[2],
            7 => 7,
            _ => unreachable!("Should not appear anywhere!"),
        };

        let mut jmp = false;

        match instruction {
            0 => register[0] = register[0] / 2_i64.pow(combo as u32),
            1 => {
                let b = register[1];
                register[1] = b ^ literal;
            }
            2 => register[1] = combo % 8,
            3 => {
                if register[0] != 0 {
                    pointer = literal as usize;
                    jmp = true;
                }
            }
            4 => {
                let b = register[1];
                let c = register[2];
                register[1] = b ^ c;
            }
            5 => out.push(combo as usize % 8),
            6 => register[1] = register[0] / 2_i64.pow(combo as u32),
            7 => register[2] = register[0] / 2_i64.pow(combo as u32),
            _ => unreachable!("Invalid instruction!"),
        }

        if !jmp {
            pointer += 2;
        }
    }
}

fn find_starting_value(mut a: i64, instructions: &Vec<usize>, index: usize) -> Vec<i64> {
    let n = instructions.len();

    if index == n {
        return vec![a];
    }

    a = a << 3;
    let mut solutions = Vec::new();

    for i in 0..8 {
        let new_a = a + i;
        let mut b = (new_a % 8) ^ 3;
        b = (b ^ (new_a >> b) ^ 5) % 8;

        if b as usize == instructions[n - 1 - index] {
            solutions.extend(find_starting_value(new_a, instructions, index + 1));
        }
    }

    solutions
}
