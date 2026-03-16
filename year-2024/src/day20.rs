use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Error> {
    // let path = "input/day20-test.txt";
    let path = "input/day20.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;
    let mut obstacles = HashSet::new();

    let mut row = 0;

    for line in buffered.lines() {
        let line_ok = line?;

        let _ = line_ok
            .trim()
            .chars()
            .enumerate()
            .for_each(|(c, x)| match x {
                '#' => {
                    obstacles.insert(Position {
                        x: c as i64,
                        y: row,
                    });
                }
                'S' => {
                    start = Some(Position {
                        x: c as i64,
                        y: row,
                    });
                }
                'E' => {
                    end = Some(Position {
                        x: c as i64,
                        y: row,
                    });
                }
                _ => {}
            });

        row += 1;
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let (part_one, part_two) = find_skips(&start, &end, &obstacles);

    println!("--- Day 20: Race Condition ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

fn racetrack_order(
    start: &Position,
    end: &Position,
    obstacles: &HashSet<Position>,
) -> Vec<Position> {
    let mut seen = HashSet::new();
    let mut racetrack = Vec::new();

    let mut queue = Vec::new();
    queue.push(Position {
        x: start.x,
        y: start.y,
    });

    loop {
        match queue.pop() {
            Some(pos) => {
                if seen.contains(&pos) {
                    continue;
                } else {
                    seen.insert(pos.clone());
                    racetrack.push(pos.clone());
                }

                if pos == *end {
                    break;
                }

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let new_pos = Position {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    };

                    if !obstacles.contains(&new_pos) {
                        queue.push(new_pos);
                    }
                }
            }
            None => break,
        }
    }

    racetrack
}

fn find_valid_shortcuts(
    pos: &Position,
    racetrack_hash: &HashSet<Position>,
    skip_size: i64,
) -> Vec<(Position, i64)> {
    let mut valid_ends = Vec::new();

    let low = -skip_size - 1;
    let high = skip_size + 1;

    for x in low..high {
        for y in low..high {
            if x.abs() + y.abs() > skip_size {
                continue;
            } else {
                let new_pos = Position {
                    x: pos.x + x,
                    y: pos.y + y,
                };
                if racetrack_hash.contains(&new_pos) {
                    valid_ends.push((new_pos, x.abs() + y.abs()));
                }
            }
        }
    }

    valid_ends
}

fn find_best_shortcut(
    pos: &Position,
    racetrack_map: &HashMap<Position, usize>,
    racetrack_hash: &HashSet<Position>,
    skip_size: i64,
) -> Vec<usize> {
    let ends = find_valid_shortcuts(pos, racetrack_hash, skip_size);
    let start_index = racetrack_map.get(&pos).unwrap();

    let shortcuts: Vec<usize> = ends
        .iter()
        .map(|(e, l)| {
            let end_index = racetrack_map.get(&e).unwrap();
            if end_index <= start_index {
                0
            } else {
                end_index - start_index - *l as usize
            }
        })
        .filter(|x| *x > 99)
        .collect();

    shortcuts
}

fn find_skips(start: &Position, end: &Position, obstacles: &HashSet<Position>) -> (usize, usize) {
    let racetrack = racetrack_order(start, end, obstacles);
    let racetrack_len = racetrack.len() - 1;
    let racetrack_hash: HashSet<Position> = racetrack.iter().cloned().collect();
    let racetrack_map: HashMap<Position, usize> =
        racetrack.iter().enumerate().map(|(e, v)| (*v, e)).collect();

    let mut shortcuts_2 = Vec::new();
    let mut shortcuts_20 = Vec::new();

    for i in 0..racetrack_len {
        shortcuts_2.extend(find_best_shortcut(
            &racetrack[i],
            &racetrack_map,
            &racetrack_hash,
            2,
        ));
        shortcuts_20.extend(find_best_shortcut(
            &racetrack[i],
            &racetrack_map,
            &racetrack_hash,
            20,
        ));
    }

    (shortcuts_2.len(), shortcuts_20.len())
}
