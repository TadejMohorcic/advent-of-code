use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day07-test.txt";
    let path = "input/day07.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut splitter_locations = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let splitter_row: Vec<u64> = line_ok.trim().chars().enumerate().filter_map(|(i, x)| (x == '^').then_some(i as u64)).collect();

        if !splitter_row.is_empty() {
            splitter_locations.push(splitter_row);
        }
    }

    let (part_one, part_two) = move_in_manifold(&splitter_locations);

    println!("--- Day 7: Laboratories ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn move_in_manifold(locations: &[Vec<u64>]) -> (u64, u64) {
    let mut number_of_splits = 0;

    let starting_position = locations[0][0];

    let mut beams = vec![0; 2 * (starting_position as usize) + 1];
    beams[starting_position as usize] = 1;

    for i in 0..locations.len() {
        let mut new_beams = vec![0; 2 * (starting_position as usize) + 1];

        let beam_ids: Vec<usize> = beams.iter().enumerate().filter_map(|(i, x)| (*x > 0).then_some(i)).collect();

        for b in beam_ids {
            if locations[i].contains(&(b as u64)) {
                new_beams[b - 1] += beams[b];
                new_beams[b + 1] += beams[b];
                new_beams[b] = 0;
                number_of_splits += 1;
            }
            else {
                new_beams[b] += beams[b];
            }
        }

        beams = new_beams;
    }

    let total_timelines = beams.iter().fold(0, |acc, x| acc + x);

    (number_of_splits, total_timelines)
}