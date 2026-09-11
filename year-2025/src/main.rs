use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

type Solver = fn() -> io::Result<()>;

fn main() {
    let days: &[(&str, Solver)] = &[
        ("Day 1", day01::main),
        ("Day 2", day02::main),
        ("Day 3", day03::main),
        ("Day 4", day04::main),
        ("Day 5", day05::main),
        ("Day 6", day06::main),
        ("Day 7", day07::main),
        ("Day 8", day08::main),
        ("Day 9", day09::main),
        ("Day 10", day10::main),
        ("Day 11", day11::main),
        ("Day 12", day12::main),
    ];

    for (name, run) in days {
        if let Err(e) = run() {
            eprintln!("{} failed: {}\n", name, e);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
