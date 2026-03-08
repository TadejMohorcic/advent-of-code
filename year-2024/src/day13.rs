use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day13-test.txt";
    let path = "input/day13.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut problems = Vec::new();

    let mut button = [[0; 2]; 2];
    let mut target = [0; 2];

    for line in buffered.lines() {
        let line_ok = line?;

        if let Some(line_split) = line_ok.trim().split_once(": ") {
            match line_split.0 {
                "Button A" => {
                    let coords: Vec<&str> = line_split.1.split(", ").collect();
                    let move_x: i64 = coords[0].split("+").nth(1).unwrap().parse().unwrap();
                    let move_y: i64 = coords[1].split("+").nth(1).unwrap().parse().unwrap();

                    button[0][0] = move_x;
                    button[1][0] = move_y;
                }
                "Button B" => {
                    let coords: Vec<&str> = line_split.1.split(", ").collect();
                    let move_x: i64 = coords[0].split("+").nth(1).unwrap().parse().unwrap();
                    let move_y: i64 = coords[1].split("+").nth(1).unwrap().parse().unwrap();

                    button[0][1] = move_x;
                    button[1][1] = move_y;
                }
                "Prize" => {
                    let coords: Vec<&str> = line_split.1.split(", ").collect();
                    let target_x: i64 = coords[0].split("=").nth(1).unwrap().parse().unwrap();
                    let target_y: i64 = coords[1].split("=").nth(1).unwrap().parse().unwrap();

                    target[0] = target_x;
                    target[1] = target_y;

                    problems.push(Problem {
                        a: button,
                        y: target,
                    });
                    button = [[0; 2]; 2];
                    target = [0; 2];
                }
                _ => unreachable!("Invalid input file format!"),
            }
        }
    }

    let part_one = solve_problems(&problems, false);
    let part_two = solve_problems(&problems, true);

    println!("--- Day 13: Claw Contraption ---");
    println!(" - Part ne solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug)]
struct Problem {
    a: [[i64; 2]; 2],
    y: [i64; 2],
}

fn solve_problem(problem: &Problem, part: bool) -> i64 {
    let a = problem.a;
    let mut y = problem.y;

    if part {
        y[0] += 10_000_000_000_000;
        y[1] += 10_000_000_000_000;
    }

    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    let m = y[0] * a[1][1] - y[1] * a[0][1];
    let n = a[0][0] * y[1] - a[1][0] * y[0];

    if det != 0 && m % det == 0 && n % det == 0 {
        return 3 * m / det + n / det;
    }

    0
}

fn solve_problems(problems: &Vec<Problem>, part: bool) -> i64 {
    problems.iter().map(|x| solve_problem(x, part)).sum()
}
