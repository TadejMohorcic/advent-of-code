use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    let path = "input/day12.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut part_one = 0;

    let tile_areas: Vec<u64> = [5, 7, 6, 7, 7, 7].to_vec();

    for line in buffered.lines() {
        let line_ok = line?;
        let split_line = line_ok.trim().split_once(": ");

        match split_line {
            Some(split) => {
                let area: u64 = split
                    .0
                    .split('x')
                    .map(|x| x.parse().unwrap())
                    .fold(1, |acc, x: u64| acc * x);
                let tiles_used: Vec<u64> = split
                    .1
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let tile_area = tiles_used
                    .iter()
                    .zip(tile_areas.iter())
                    .fold(0, |acc, (x, y)| acc + x * y);
                if area > tile_area {
                    part_one += 1;
                }
            }
            None => continue,
        }
    }

    println!("--- Day 12: Christmas Tree Farm ---");
    println!(" - Part one solution: {}", part_one);
    println!("");

    Ok(())
}
