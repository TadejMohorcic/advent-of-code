use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn main() -> Result<(), Error> {
    // let path = "input/day16-test.txt";
    let path = "input/day16.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut start: Option<Reindeer> = None;
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
                    start = Some(Reindeer {
                        x: c as i64,
                        y: row,
                        dx: 1,
                        dy: 0,
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

    let (part_one, part_two) = traverse_maze(start.unwrap(), end.unwrap(), &obstacles);

    println!("--- Day 16: Reindeer Maze ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Reindeer {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

#[derive(Eq, PartialEq, Clone)]
struct Node(i64, Reindeer, HashSet<Position>);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rotate(dx: i64, dy: i64) -> Vec<(i64, i64)> {
    match (dx, dy) {
        (_, 0) => return [(0, 1), (0, -1)].to_vec(),
        (0, _) => return [(1, 0), (-1, 0)].to_vec(),
        _ => unreachable!("We only have 4 directions!"),
    }
}

fn traverse_maze(start: Reindeer, end: Position, obstacles: &HashSet<Position>) -> (i64, usize) {
    let mut queue = BinaryHeap::new();
    let seen = HashSet::new();
    queue.push(Reverse(Node(0, start, seen)));

    let mut visited = HashMap::new();

    let mut best_path: Option<i64> = None;
    let mut tiles_visited = HashSet::new();

    loop {
        match queue.pop() {
            Some(Reverse(Node(cost, pos, tiles))) => {
                if (Position { x: pos.x, y: pos.y }) == end {
                    if best_path.is_none() {
                        best_path = Some(cost);
                    }

                    tiles_visited.extend(tiles);
                    continue;
                }

                if !best_path.is_none() && cost > best_path.unwrap() {
                    break;
                }

                if let Some(c) = visited.get(&pos) {
                    if cost > *c {
                        continue;
                    }
                } else {
                    visited.insert(
                        Reindeer {
                            x: pos.x,
                            y: pos.y,
                            dx: pos.dx,
                            dy: pos.dy,
                        },
                        cost,
                    );
                }

                let new_x = pos.x + pos.dx;
                let new_y = pos.y + pos.dy;

                if !obstacles.contains(&Position { x: new_x, y: new_y }) {
                    let mut new_tiles = tiles.clone();
                    new_tiles.insert(Position { x: pos.x, y: pos.y });

                    queue.push(Reverse(Node(
                        cost + 1,
                        Reindeer {
                            x: new_x,
                            y: new_y,
                            dx: pos.dx,
                            dy: pos.dy,
                        },
                        new_tiles,
                    )));
                }

                for (dx, dy) in rotate(pos.dx, pos.dy) {
                    let mut new_tiles = tiles.clone();
                    new_tiles.insert(Position { x: pos.x, y: pos.y });

                    queue.push(Reverse(Node(
                        cost + 1000,
                        Reindeer {
                            x: pos.x,
                            y: pos.y,
                            dx: dx,
                            dy: dy,
                        },
                        new_tiles,
                    )));
                }
            }
            None => return (0, 0),
        }
    }

    (best_path.unwrap(), tiles_visited.len() + 1)
}
