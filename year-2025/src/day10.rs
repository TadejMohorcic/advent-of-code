use std::fs::File;
use std::io::{self, BufReader, BufRead, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day10_test.txt";
    let path = "input/day10.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let line_ok = line?;
        println!("{:?}", line_ok);
    }

    let part_one = 0;
    let part_two = 0;

    println!("--- Day 10: Factory ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}