use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Error> {
    let path = "input/day08.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut junction_boxes = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let p: Vec<i64> = line_ok
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let position = Position {
            x: p[0],
            y: p[1],
            z: p[2],
        };

        junction_boxes.push(position);
    }

    let (part_one, part_two) = connect_boxes(&junction_boxes, 1000);

    println!("--- Day 8: Playground ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn calculate_distance(p1: Position, p2: Position) -> i64 {
    let distance = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2);

    distance
}

fn get_distances_sorted(positions: &[Position]) -> Vec<(i64, usize, usize)> {
    let n = positions.len();
    let mut distances = Vec::new();

    for i in 0..n {
        let p1 = positions[i];
        for j in i + 1..n {
            let p2 = positions[j];
            let distance = calculate_distance(p1, p2);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by_key(|t| t.0);

    distances
}

fn connect_boxes(positions: &[Position], num_of_steps: usize) -> (i64, i64) {
    let mut part_one = 1;
    let mut part_two = 1;

    let ds = get_distances_sorted(positions);

    let mut connections = HashMap::new();

    for (i, _) in positions.iter().enumerate() {
        let mut connected_to = HashSet::new();
        connected_to.insert(i);
        connections.insert(i, connected_to);
    }

    let mut steps = 0;

    for (_, i, j) in ds {
        steps += 1;

        if connections[&i].contains(&j) {
            continue;
        }

        let connected_union: HashSet<usize> = connections
            .get(&i)
            .iter()
            .flat_map(|s| s.iter().copied())
            .chain(connections.get(&j).iter().flat_map(|s| s.iter().copied()))
            .collect();

        for k in &connected_union {
            connections.insert(*k, connected_union.clone());
        }

        if connected_union.len() == connections.len() {
            let p1 = positions[i];
            let p2 = positions[j];
            part_two *= p1.x * p2.x;
            break;
        }

        if steps == num_of_steps {
            let mut groups: Vec<usize> = connections.values().map(|x| x.len()).collect();
            groups.sort_by(|a, b| b.cmp(&a));

            let mut index = 0;

            for _ in 0..3 {
                part_one *= groups[index] as i64;
                index += groups[index];
            }
        }
    }

    (part_one, part_two)
}
