use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

pub fn main() -> Result<(), Error> {
    // let path = "input/day15-test.txt";
    let path = "input/day15.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut t = MapType::Grid;

    let mut obstacles = HashSet::new();
    let mut boxes = HashSet::new();
    let mut start: Option<Position> = None;

    let mut instructions = Vec::new();

    let mut row = 0;

    for line in buffered.lines() {
        let line_ok = line?;

        if line_ok.is_empty() {
            t = MapType::Instructions;
            continue;
        }

        match t {
            MapType::Grid => {
                let grid_row: Vec<(usize, char)> = line_ok.trim().chars().enumerate().collect();

                obstacles.extend(
                    &mut grid_row
                        .iter()
                        .filter(|(_, x)| *x == '#')
                        .map(|(c, _)| Position {
                            x: *c as i32,
                            y: row,
                        }),
                );

                boxes.extend(
                    &mut grid_row
                        .iter()
                        .filter(|(_, x)| *x == 'O')
                        .map(|(c, _)| Position {
                            x: *c as i32,
                            y: row,
                        }),
                );

                if let Some((col, _)) = grid_row.iter().filter(|(_, x)| *x == '@').next() {
                    start = Some(Position {
                        x: *col as i32,
                        y: row,
                    });
                }

                row += 1;
            }
            MapType::Instructions => {
                let grid_row: Vec<char> = line_ok.trim().chars().collect();
                instructions.extend(&grid_row);
            }
        }
    }

    let start = start.unwrap();

    let part_one = move_robot(start.clone(), &obstacles, &boxes, &instructions, false);
    let part_two = move_robot(start, &obstacles, &boxes, &instructions, true);

    println!("--- Day 15: Warehouse Woes ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

enum MapType {
    Grid,
    Instructions,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&self, dir: (i32, i32)) -> Position {
        Position {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}

fn get_direction(instruction: &char) -> (i32, i32) {
    match instruction {
        '>' => return (1, 0),
        '<' => return (-1, 0),
        '^' => return (0, -1),
        'v' => return (0, 1),
        _ => return (0, 0),
    }
}

fn move_boxes(
    pos: &Position,
    obstacles: &HashSet<Position>,
    boxes: &mut HashSet<Position>,
    dir: (i32, i32),
) -> bool {
    let empty = Position {
        x: pos.x + dir.0,
        y: pos.y + dir.1,
    };

    let can_move = !obstacles.contains(&empty)
        && (!boxes.contains(&empty) || move_boxes(&empty, obstacles, boxes, dir));

    if can_move {
        boxes.remove(&pos);
        boxes.insert(empty);
        return true;
    } else {
        return false;
    }
}

fn positions_to_check(pos: &Position, dir: (i32, i32)) -> Vec<Position> {
    let mut to_check = Vec::new();

    match dir {
        (0, _) => {
            to_check.push(Position {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            });
            to_check.push(Position {
                x: pos.x + dir.0 - 1,
                y: pos.y + dir.1,
            })
        }
        (1, 0) => to_check.push(Position {
            x: pos.x + dir.0,
            y: pos.y + dir.1,
        }),
        (-1, 0) => to_check.push(Position {
            x: pos.x + 2 * dir.0,
            y: pos.y + 2 * dir.1,
        }),
        _ => unreachable!("Should never happen!"),
    }

    to_check
}

fn boxes_to_check(pos: &Position, dir: (i32, i32)) -> Vec<Position> {
    let mut to_check = Vec::new();

    match dir {
        (0, _) => {
            to_check.push(Position {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            });
            to_check.push(Position {
                x: pos.x + dir.0 - 1,
                y: pos.y + dir.1,
            });
            to_check.push(Position {
                x: pos.x + dir.0 + 1,
                y: pos.y + dir.1,
            })
        }
        (_, 0) => to_check.push(Position {
            x: pos.x + 2 * dir.0,
            y: pos.y + 2 * dir.1,
        }),
        _ => unreachable!("Should never happen!"),
    }

    to_check
}

fn move_wide_boxes(pos: &Position, boxes: &mut HashSet<Position>, dir: (i32, i32)) {
    let to_check = boxes_to_check(pos, dir);

    for position in &to_check {
        if boxes.contains(&position) {
            move_wide_boxes(position, boxes, dir);
        }
    }

    boxes.remove(pos);
    boxes.insert(Position {
        x: pos.x + dir.0,
        y: pos.y + dir.1,
    });
}

fn check_wide_boxes(
    pos: &Position,
    obstacles: &HashSet<Position>,
    boxes: &mut HashSet<Position>,
    dir: (i32, i32),
) -> bool {
    let to_check = boxes_to_check(pos, dir);

    let can_move = to_check.iter().all(|x| {
        !obstacles.contains(x) && (!boxes.contains(x) || check_wide_boxes(x, obstacles, boxes, dir))
    });

    can_move
}

fn move_robot(
    start: Position,
    obstacles: &HashSet<Position>,
    boxes: &HashSet<Position>,
    instructions: &Vec<char>,
    part: bool,
) -> i32 {
    let mut pos = if part {
        Position {
            x: 2 * start.x,
            y: start.y,
        }
    } else {
        start
    };

    let mut boxes = if part {
        boxes
            .iter()
            .map(|x| Position { x: 2 * x.x, y: x.y })
            .collect()
    } else {
        boxes.clone()
    };

    let obstacles = if part {
        obstacles
            .iter()
            .map(|x| Position { x: 2 * x.x, y: x.y })
            .collect()
    } else {
        obstacles.clone()
    };

    for instruction in instructions {
        let dir = get_direction(instruction);

        if part {
            let to_check = positions_to_check(&pos, dir);

            let can_move: bool = to_check.iter().all(|x| {
                !obstacles.contains(x)
                    && (!boxes.contains(x) || check_wide_boxes(x, &obstacles, &mut boxes, dir))
            });

            if can_move {
                for position in &to_check {
                    if boxes.contains(position) {
                        move_wide_boxes(position, &mut boxes, dir);
                    }
                }

                pos = pos.step(dir);
            }
        } else {
            let empty = Position {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            };

            let can_move = !obstacles.contains(&empty)
                && (!boxes.contains(&empty) || move_boxes(&empty, &obstacles, &mut boxes, dir));

            if can_move {
                pos = pos.step(dir);
            }
        }
    }

    boxes.iter().map(|x| 100 * x.y + x.x).sum()
}
