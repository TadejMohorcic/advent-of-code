use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::{HashSet, HashMap};

pub fn main() -> Result<(), Error> {
    // let path = "input/day08-test.txt";
    let path = "input/day08.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut antenna_collection: HashMap<char, Vec<Position>> = HashMap::new();

    let mut row = 0;

    for line in buffered.lines() {
        let line_ok = line?;
        let antennas: Vec<(usize, char)> = line_ok.trim().chars().enumerate().filter(|(_, c)| *c != '.').collect();

        if !antennas.is_empty() {
            for (col, antenna) in &antennas {
                let position = Position{x: *col as i64, y: row};

                if let Some(vec) = antenna_collection.get_mut(antenna) {
                    vec.push(position);
                }
                else {
                    let mut new_vec = Vec::new();
                    new_vec.push(position);

                    antenna_collection.insert(*antenna, new_vec);
                }
            }
        }

        row += 1;
    }

    let part_one = unique_locations(&antenna_collection, row, false);
    let part_two = unique_locations(&antenna_collection, row, true);

    println!("--- Day 8: Resonant Collinearity ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Hash, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

fn find_antinodes(antennas: &Vec<Position>, max_dim: i64, part: bool) -> HashSet<Position> {
    let mut antinode_positions = HashSet::new();
    let n = antennas.len();

    for i in 0..n {
        let antenna_a = &antennas[i];

        for j in i+1..n {
            let antenna_b = &antennas[j];

            let dx = antenna_a.x - antenna_b.x;
            let dy = antenna_a.y - antenna_b.y;

            if part {
                for (antenna, sign) in [(antenna_a, 1), (antenna_b, -1)] {
                    let mut x = antenna.x;
                    let mut y = antenna.y;

                    loop {
                        if 0 <= x && x < max_dim && 0 <= y && y < max_dim {
                            antinode_positions.insert(Position {x: x, y: y});
                        }
                        else {
                            break;
                        }

                        x += sign * dx;
                        y += sign * dy;
                    }
                }
            }
            else {
                for (antenna, sign) in [(antenna_a, 1), (antenna_b, -1)] {
                    let pos = Position {x: antenna.x + sign * dx, y: antenna.y + sign * dy};

                    if 0 <= pos.x && pos.x < max_dim && 0 <= pos.y && pos.y < max_dim {
                        antinode_positions.insert(pos);
                    }
                }
            }
        }
    }

    antinode_positions
}

fn unique_locations(antennas: &HashMap<char, Vec<Position>>, max_dim: i64, part: bool) -> usize {
    antennas.iter().flat_map(|(_, v)| find_antinodes(&v, max_dim, part)).collect::<HashSet<_>>().len()
}