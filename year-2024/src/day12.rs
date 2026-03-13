use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

pub fn main() -> Result<(), Error> {
    // let path = "input/day12-test.txt";
    let path = "input/day12.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut map = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let map_row: Vec<char> = line_ok.trim().chars().collect();
        map.push(map_row);
    }

    let (part_one, part_two) = calculate_price(&map);

    println!("--- Day 12: Garden Groups ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

fn flood_fill(
    pos: Position,
    map: &Vec<Vec<char>>,
    ch: char,
    visited: &mut HashSet<Position>,
) -> (usize, usize, usize) {
    let m = map.len();
    let n = map[0].len();

    let mut area = 1;
    let mut border = 0;
    let mut sides = 0;

    let row = pos.row;
    let col = pos.col;
    let r = row as i64;
    let c = col as i64;

    visited.insert(pos);

    let in_shape = |r: i64, c: i64| -> bool {
        r >= 0 && c >= 0 && m > r as usize && n > c as usize && map[r as usize][c as usize] == ch
    };

    if !in_shape(r - 1, c) && (!in_shape(r, c - 1) || in_shape(r - 1, c - 1)) {
        sides += 1;
    }
    if !in_shape(r + 1, c) && (!in_shape(r, c - 1) || in_shape(r + 1, c - 1)) {
        sides += 1;
    }
    if !in_shape(r, c - 1) && (!in_shape(r - 1, c) || in_shape(r - 1, c - 1)) {
        sides += 1;
    }
    if !in_shape(r, c + 1) && (!in_shape(r - 1, c) || in_shape(r - 1, c + 1)) {
        sides += 1;
    }

    for dir in [-1, 1] {
        let new_row = row as i64 + dir;
        let new_col = col as i64 + dir;

        if in_shape(new_row, c) {
            if !visited.contains(&Position {
                row: new_row as usize,
                col: col,
            }) {
                let (a, b, s) = flood_fill(
                    Position {
                        row: new_row as usize,
                        col: col,
                    },
                    map,
                    ch,
                    visited,
                );
                area += a;
                border += b;
                sides += s;
            }
        } else {
            border += 1;
        }

        if in_shape(r, new_col) {
            if !visited.contains(&Position {
                row: row,
                col: new_col as usize,
            }) {
                let (a, b, s) = flood_fill(
                    Position {
                        row: row,
                        col: new_col as usize,
                    },
                    map,
                    ch,
                    visited,
                );
                area += a;
                border += b;
                sides += s;
            }
        } else {
            border += 1;
        }
    }

    (area, border, sides)
}

fn calculate_price(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut price_border = 0;
    let mut price_sides = 0;
    let mut visited = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let pos = Position { row: i, col: j };
            let ch = map[i][j];
            if !visited.contains(&pos) {
                let (area, border, sides) = flood_fill(pos, map, ch, &mut visited);
                price_border += area * border;
                price_sides += area * sides;
            }
        }
    }

    (price_border, price_sides)
}
