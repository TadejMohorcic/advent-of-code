use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

pub fn main() -> Result<(), Error> {
    // let path = "input/day10-test.txt";
    let path = "input/day10.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut topological_map = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let heights: Vec<u32> = line_ok
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        topological_map.push(heights);
    }

    let part_one = score_trailheads(&topological_map, false);
    let part_two = score_trailheads(&topological_map, true);

    println!("--- Day 10: Hoof It ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn score_trail(
    position: Position,
    height: u32,
    map: &Vec<Vec<u32>>,
    peaks_reached: &mut HashSet<Position>,
) -> usize {
    let m = map.len();
    let n = map[0].len();

    let x = position.x;
    let y = position.y;

    if height == 9 {
        peaks_reached.insert(Position { x: x, y: y });
        return 1;
    }

    let mut total_trails = 0;

    for i in [-1, 1] {
        let new_x = x as i64 + i;
        let new_y = y as i64 + i;

        if 0 <= new_x && n > new_x as usize && map[y][new_x as usize] == height + 1 {
            total_trails += score_trail(
                Position {
                    x: new_x as usize,
                    y: y,
                },
                height + 1,
                map,
                peaks_reached,
            );
        }

        if 0 <= new_y && m > new_y as usize && map[new_y as usize][x] == height + 1 {
            total_trails += score_trail(
                Position {
                    x: x,
                    y: new_y as usize,
                },
                height + 1,
                map,
                peaks_reached,
            );
        }
    }

    total_trails
}

fn score_trailheads(map: &Vec<Vec<u32>>, part: bool) -> usize {
    let mut result = 0;
    let mut row = 0;

    for map_row in map {
        result += map_row
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == 0)
            .map(|(col, _)| {
                let mut peaks = HashSet::new();
                let trails = score_trail(Position { x: col, y: row }, 0, map, &mut peaks);
                if part { trails } else { peaks.len() }
            })
            .sum::<usize>();

        row += 1;
    }

    result
}
