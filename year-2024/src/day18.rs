use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{BinaryHeap, HashSet};

pub fn main() -> Result<(), Error> {
    // let path = "input/day18-test.txt";
    let path = "input/day18.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut positions = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let location = line_ok.trim().split_once(',').unwrap();
        let pos = Position {
            x: location.0.parse().expect("Should be a number."),
            y: location.1.parse().expect("Should be a number."),
        };

        positions.push(pos);
    }

    let part_one = navigate_maze(&positions[..1024]);
    let part_two = smallest_break(&positions);

    println!("--- Day 18: RAM Run ---");
    println!(" - Part one solution: {}", part_one.unwrap());
    println!(" - Part two solution: {:?}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

fn is_valid_pos(x: i64, y: i64, w: i64, h: i64) -> bool {
    return 0 <= x && x <= w && 0 <= y && y <= h;
}

fn navigate_maze(obstacles: &[Position]) -> Option<i64> {
    const WIDTH: i64 = 70;
    const HEIGHT: i64 = 70;

    let mut obstacles_hash: HashSet<Position> = obstacles.iter().cloned().collect();

    let mut queue = BinaryHeap::new();
    queue.push((0, 0, 0));

    loop {
        match queue.pop() {
            Some((cost, x, y)) => {
                let current_pos = Position { x: x, y: y };

                if obstacles_hash.contains(&current_pos) {
                    continue;
                } else {
                    obstacles_hash.insert(current_pos);
                }

                if x == WIDTH && y == HEIGHT {
                    return Some(-1 * cost);
                }

                if is_valid_pos(x, y, WIDTH, HEIGHT) {
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        queue.push((cost - 1, x + dx, y + dy));
                    }
                }
            }
            None => break,
        }
    }

    return None;
}

fn smallest_break(obstacles: &Vec<Position>) -> (i64, i64) {
    let n = obstacles.len();

    let mut pointers = [0, n];

    loop {
        let mid = (pointers[0] + pointers[1]) / 2;

        if pointers[0] == mid {
            return (obstacles[mid].x, obstacles[mid].y);
        }

        let can_navigate = navigate_maze(&obstacles[..mid]);
        match can_navigate {
            Some(_) => {
                pointers[0] = mid;
                continue;
            }
            None => {
                pointers[1] = mid;
                continue;
            }
        }
    }
}
