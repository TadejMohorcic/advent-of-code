use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn main() {
    let boxes = parse_input("input/day08.txt");
    let (part_one, part_two) = connect_boxes(&boxes, 1000);

    println!("--- Day 8: Playground ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Position> {
    let mut boxes = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut position = line.trim().split(',').map(|x| x.parse::<i64>().unwrap());
            boxes.push(Position {
                x: position.next().unwrap(),
                y: position.next().unwrap(),
                z: position.next().unwrap(),
            })
        }
    }

    boxes
}

fn squared_distance(p1: Position, p2: Position) -> i64 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)
}

fn get_sorted_distances(positions: &[Position]) -> Vec<(i64, usize, usize)> {
    let n = positions.len();
    let mut distances = Vec::new();

    for i in 0..n {
        let p1 = positions[i];

        for j in i + 1..n {
            let p2 = positions[j];
            let distance = squared_distance(p1, p2);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by_key(|t| t.0);

    distances
}

fn connect_boxes(positions: &[Position], desired_steps: usize) -> (i64, i64) {
    let mut part_one = 1;
    let mut part_two = 1;
    let sorted_distances = get_sorted_distances(positions);
    let mut connections = HashMap::new();

    for (i, _) in positions.iter().enumerate() {
        let mut connected_to = HashSet::new();
        connected_to.insert(i);
        connections.insert(i, connected_to);
    }

    let mut steps = 0;

    for (_, i, j) in sorted_distances {
        if steps == desired_steps {
            let mut groups: Vec<usize> = connections.values().map(|x| x.len()).collect();
            groups.sort_by(|a, b| b.cmp(&a));
            let mut index = 0;

            for _ in 0..3 {
                part_one *= groups[index] as i64;
                index += groups[index];
            }
        }

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
            part_two *= positions[i].x * positions[j].x;
            break;
        }
    }

    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let boxes = parse_input("input/day08-test.txt");
        let (part_one, _) = connect_boxes(&boxes, 10);
        assert_eq!(part_one, 40)
    }

    #[test]
    fn part_two_example() {
        let boxes = parse_input("input/day08-test.txt");
        let (_, part_two) = connect_boxes(&boxes, 10);
        assert_eq!(part_two, 25272)
    }
}
