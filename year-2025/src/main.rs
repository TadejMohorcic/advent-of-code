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

fn main() {
    day01::main();
    day02::main();
    day03::main();
    day04::main();
    day05::main();
    let _ = day06::main();
    day07::main();
    let _ = day08::main();
    let _ = day09::main();
    let _ = day10::main();
    let _ = day11::main();
    let _ = day12::main();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
