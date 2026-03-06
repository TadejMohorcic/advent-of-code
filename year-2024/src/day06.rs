use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::cmp::{max, min};
use std::collections::HashSet;

use rayon::prelude::*;

pub fn main() -> Result<(), Error> {
    // let path = "input/day06-test.txt";
    let path = "input/day06.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut guard_position: Option<CurrentPosition> = None;

    let mut locations_by_row = Vec::new();
    let mut locations_by_col = Vec::new();

    let mut row = 0;

    for line in buffered.lines() {
        let line_ok = line?;

        let n = line_ok.len();

        if row == 0 {
            for _ in 0..n {
                locations_by_col.push(Vec::new());
            }
        }

        let guard: Vec<i32> = line_ok
            .trim()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(s, _)| s as i32)
            .collect();

        if !guard.is_empty() {
            let position = CurrentPosition {
                x: guard[0],
                y: row,
                dx: 0,
                dy: -1,
            };
            guard_position = Some(position);
        }

        let obstacles: Vec<i32> = line_ok
            .trim()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(s, _)| s as i32)
            .collect();

        for obstacle in &obstacles {
            locations_by_col[*obstacle as usize].push(row);
        }

        locations_by_row.push(obstacles);

        row += 1;
    }

    let starting_pos = guard_position.unwrap();

    let (part_one, visited) = guard_positions(&starting_pos, &locations_by_row, &locations_by_col);
    let part_two = check_loops(&starting_pos, &locations_by_row, &locations_by_col, visited);

    println!("--- Day 6: Guard Gallivant ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct CurrentPosition {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn guard_positions(
    starting_pos: &CurrentPosition,
    obstacles_row: &[Vec<i32>],
    obstacles_col: &[Vec<i32>],
) -> (usize, HashSet<Position>) {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut current_pos = starting_pos.clone();

    let n = obstacles_row.len();

    loop {
        let x = current_pos.x;
        let y = current_pos.y;
        let dx = current_pos.dx;
        let dy = current_pos.dy;

        let dir = if dx != 0 { dx } else { dy };
        let position = if dir == dx { x } else { y };

        let search_space = if dir == dx {
            &obstacles_row[y as usize]
        } else {
            &obstacles_col[x as usize]
        };

        if let Some(next_pos) = find_next_obstacle(search_space, position, dir) {
            let min_pos = min(position, next_pos);
            let max_pos = max(position, next_pos);

            for i in min_pos..=max_pos {
                let to_add = if dir == dx {
                    Position { x: i as i32, y: y }
                } else {
                    Position { x: x, y: i as i32 }
                };
                visited.insert(to_add);
            }

            let new_dir = next_direction(dx, dy);

            current_pos = if dir == dx {
                CurrentPosition {
                    x: next_pos,
                    y: y,
                    dx: new_dir.0,
                    dy: new_dir.1,
                }
            } else {
                CurrentPosition {
                    x: x,
                    y: next_pos,
                    dx: new_dir.0,
                    dy: new_dir.1,
                }
            };
        } else {
            if dir > 0 {
                for i in position as usize..n {
                    let to_add = if dir == dx {
                        Position { x: i as i32, y: y }
                    } else {
                        Position { x: x, y: i as i32 }
                    };
                    visited.insert(to_add);
                }
            } else {
                for i in 0..position as usize {
                    let to_add = if dir == dx {
                        Position { x: i as i32, y: y }
                    } else {
                        Position { x: x, y: i as i32 }
                    };
                    visited.insert(to_add);
                }
            }

            break;
        }
    }

    (visited.len(), visited)
}

fn find_next_obstacle(search_space: &Vec<i32>, pos: i32, dir: i32) -> Option<i32> {
    if dir > 0 {
        let obstacles: Vec<&i32> = search_space.iter().filter(|x| **x > pos).collect();

        if !obstacles.is_empty() {
            return Some(*obstacles[0] - 1);
        }
    } else {
        let obstacles: Vec<&i32> = search_space.iter().filter(|x| **x < pos).collect();

        if !obstacles.is_empty() {
            let n = obstacles.len();
            return Some(*obstacles[n - 1] + 1);
        }
    }

    None
}

fn next_direction(dx: i32, dy: i32) -> (i32, i32) {
    let next_position = match (dx, dy) {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => (0, 0),
    };

    next_position
}

fn check_loops(
    starting_pos: &CurrentPosition,
    obstacles_row: &[Vec<i32>],
    obstacles_col: &[Vec<i32>],
    to_check: HashSet<Position>,
) -> i32 {
    to_check
        .par_iter()
        .map(|position| {
            let x = position.x as usize;
            let y = position.y as usize;
            let mut temp_row = obstacles_row.to_vec();
            let mut temp_col = obstacles_col.to_vec();

            let row_pos = temp_row[y]
                .binary_search(&(position.x))
                .unwrap_or_else(|e| e);
            temp_row[y].insert(row_pos, position.x);

            let col_pos = temp_col[x]
                .binary_search(&(position.y))
                .unwrap_or_else(|e| e);
            temp_col[x].insert(col_pos, position.y);

            loop_guard(starting_pos, &temp_row, &temp_col) as i32
        })
        .sum()
}

fn loop_guard(
    starting_pos: &CurrentPosition,
    obstacles_row: &[Vec<i32>],
    obstacles_col: &[Vec<i32>],
) -> bool {
    let mut visited: HashSet<CurrentPosition> = HashSet::new();
    let mut current_pos = starting_pos.clone();

    let n = obstacles_row.len();

    loop {
        if visited.contains(&current_pos) {
            return true;
        }

        let x = current_pos.x;
        let y = current_pos.y;
        let dx = current_pos.dx;
        let dy = current_pos.dy;

        let dir = if dx != 0 { dx } else { dy };
        let position = if dir == dx { x } else { y };

        let search_space = if dir == dx {
            &obstacles_row[y as usize]
        } else {
            &obstacles_col[x as usize]
        };

        if let Some(next_pos) = find_next_obstacle(search_space, position, dir) {
            let min_pos = min(position, next_pos);
            let max_pos = max(position, next_pos);

            for i in min_pos..=max_pos {
                let to_add = if dir == dx {
                    CurrentPosition {
                        x: i as i32,
                        y: y,
                        dx: dx,
                        dy: dy,
                    }
                } else {
                    CurrentPosition {
                        x: x,
                        y: i as i32,
                        dx: dx,
                        dy: dy,
                    }
                };

                visited.insert(to_add);
            }

            let new_dir = next_direction(dx, dy);

            current_pos = if dir == dx {
                CurrentPosition {
                    x: next_pos,
                    y: y,
                    dx: new_dir.0,
                    dy: new_dir.1,
                }
            } else {
                CurrentPosition {
                    x: x,
                    y: next_pos,
                    dx: new_dir.0,
                    dy: new_dir.1,
                }
            };
        } else {
            if dir > 0 {
                for i in position as usize..n {
                    let to_add = if dir == dx {
                        CurrentPosition {
                            x: i as i32,
                            y: y,
                            dx: dx,
                            dy: dy,
                        }
                    } else {
                        CurrentPosition {
                            x: x,
                            y: i as i32,
                            dx: dx,
                            dy: dy,
                        }
                    };

                    visited.insert(to_add);
                }
            } else {
                for i in 0..position as usize {
                    let to_add = if dir == dx {
                        CurrentPosition {
                            x: i as i32,
                            y: y,
                            dx: dx,
                            dy: dy,
                        }
                    } else {
                        CurrentPosition {
                            x: x,
                            y: i as i32,
                            dx: dx,
                            dy: dy,
                        }
                    };

                    visited.insert(to_add);
                }
            }

            break;
        }
    }

    false
}
