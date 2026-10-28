use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (obstacles, start, end) = parse_input("input/day16.txt")?;
    let (part_one, part_two) = traverse_reindeer_maze(&obstacles, start, end);

    println!("--- Day 16: Reindeer Maze ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

type Maze = (Vec<Vec<i64>>, (i64, i64), (i64, i64));

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Maze> {
    let mut start = None;
    let mut end = None;
    let mut obstacles = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for (row, line) in lines.map_while(Result::ok).enumerate() {
            if let Some(col) = line.trim().chars().position(|char| char == 'S') {
                start = Some((row as i64, col as i64));
            }

            if let Some(col) = line.trim().chars().position(|char| char == 'E') {
                end = Some((row as i64, col as i64));
            }

            obstacles.push(
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(col, char)| (char == '#').then_some(col as i64))
                    .collect(),
            );
        }
    }

    Ok((obstacles, start.unwrap(), end.unwrap()))
}

#[derive(Eq, PartialEq, Clone)]
struct Node(i64, (i64, i64, i64, i64), HashSet<(i64, i64)>);

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

fn traverse_reindeer_maze(
    obstacles: &[Vec<i64>],
    start: (i64, i64),
    end: (i64, i64),
) -> (i64, usize) {
    let mut queue = BinaryHeap::new();
    let seen = HashSet::new();
    queue.push(Reverse(Node(0, (start.0, start.1, 0, 1), seen)));
    let mut visited = HashMap::new();
    let mut best_path: Option<i64> = None;
    let mut tiles_visited = HashSet::new();

    while let Some(Reverse(Node(cost, pos, tiles))) = queue.pop() {
        if (pos.0, pos.1) == end {
            if best_path.is_none() {
                best_path = Some(cost)
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
            visited.insert(pos, cost);
        }

        let new_row = pos.0 + pos.2;
        let new_col = pos.1 + pos.3;

        if !obstacles[new_row as usize].contains(&new_col) {
            let mut new_tiles = tiles.clone();
            new_tiles.insert((pos.0, pos.1));
            queue.push(Reverse(Node(
                cost + 1,
                (new_row, new_col, pos.2, pos.3),
                new_tiles,
            )));
        }

        for (drow, dcol) in [(-pos.3, pos.2), (pos.3, -pos.2)] {
            let mut new_tiles = tiles.clone();
            new_tiles.insert((pos.0, pos.1));
            queue.push(Reverse(Node(
                cost + 1000,
                (pos.0, pos.1, drow, dcol),
                new_tiles,
            )));
        }
    }

    (best_path.unwrap(), tiles_visited.len() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (obstacles, start, end) = parse_input("input/day16-test.txt").unwrap();
        let (part_one, _) = traverse_reindeer_maze(&obstacles, start, end);
        assert_eq!(part_one, 7036);
    }

    #[test]
    fn part_two_example() {
        let (obstacles, start, end) = parse_input("input/day16-test.txt").unwrap();
        let (_, part_two) = traverse_reindeer_maze(&obstacles, start, end);
        assert_eq!(part_two, 45);
    }
}
