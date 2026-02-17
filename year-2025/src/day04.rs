use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::HashSet;

pub fn main() -> Result<(), Error> {
    // let path = "input/day04_test.txt";
    let path = "input/day04.txt"; 

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut locations = HashSet::new();
    let mut row = 0;

    for line in buffered.lines() {
        let line_ok = line?;
        let paper_rolls: Vec<Location> = line_ok.trim().chars().enumerate().filter(|(_, x)| *x == '@').map(|(i, _)| Location {row: row as i32, column: i as i32}).collect();
        row += 1;

        locations.extend(paper_rolls);
    }

    let part_one = remove_paper_rolls(&locations, true);
    let part_two = remove_paper_rolls(&locations, false);

    println!("--- Day 4: Printing Department ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Location {
    row: i32,
    column: i32
}

fn is_accessible(location: &Location, locations: &HashSet<Location>) -> bool {
    let mut neighbours = 0;

    let row = location.row;
    let column = location.column;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let paper_location = Location {row: row + i, column: column + j};

            if locations.contains(&paper_location) {
                neighbours += 1;
            }
        }
    }

    neighbours < 4
}

fn remove_paper_rolls(locations: &HashSet<Location>, do_one_step: bool) -> usize {
    let mut total_removed = 0;
    let mut locations_copy = locations.clone();

    loop {
        let mut new_locations = HashSet::new();
        let mut current_removed = 0;

        for location in &locations_copy {
            if is_accessible(location, &locations_copy) {
                current_removed += 1;
            }
            else {
                new_locations.insert(location.clone());
            }
        }

        locations_copy = new_locations;
        total_removed += current_removed;

        if do_one_step || current_removed == 0 {
            break;
        }
    }

    total_removed
}
