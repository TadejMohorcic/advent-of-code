use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let obstacles = parse_input("input/day18.txt")?;
    let maze_size = 70;
    let part_one = navigate_maze(&obstacles[..1024], maze_size);
    let part_two = smallest_break(&obstacles, maze_size);

    println!("--- Day 18: RAM Run ---");
    println!(" - Part one solution: {}", part_one.unwrap());
    println!(" - Part two solution: {:?}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<(i64, i64)>> {
    let lines = crate::read_lines(&filename)?;
    let mut positions = Vec::new();

    for line in lines.map_while(Result::ok) {
        let mut coordinates = line.trim().split(',').map(|n| n.parse::<i64>().unwrap());
        positions.push((coordinates.next().unwrap(), coordinates.next().unwrap()));
    }

    Ok(positions)
}

fn is_valid_pos(x: i64, y: i64, w: i64, h: i64, obstacles: &HashSet<(i64, i64)>) -> bool {
    0 <= x && x <= w && 0 <= y && y <= h && !obstacles.contains(&(x, y))
}

fn navigate_maze(obstacles: &[(i64, i64)], maze_size: i64) -> Option<i64> {
    let obstacles_hash: HashSet<(i64, i64)> = obstacles.iter().cloned().collect();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0)));

    while let Some(Reverse((cost, x, y))) = queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        } else {
            visited.insert((x, y));
        }

        if x == maze_size && y == maze_size {
            return Some(cost);
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if is_valid_pos(x + dx, y + dy, maze_size, maze_size, &obstacles_hash) {
                queue.push(Reverse((cost + 1, x + dx, y + dy)));
            }
        }
    }

    None
}

fn smallest_break(obstacles: &[(i64, i64)], maze_size: i64) -> String {
    let n = obstacles.len();
    let mut pointers = [0, n];

    loop {
        let mid = (pointers[0] + pointers[1]) / 2;

        if pointers[0] == mid {
            return format!("{},{}", obstacles[mid].0, obstacles[mid].1);
        }

        match navigate_maze(&obstacles[..mid], maze_size) {
            Some(_) => {
                pointers[0] = mid;
            }
            None => {
                pointers[1] = mid;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let obstacles = parse_input("input/day18-test.txt").unwrap();
        let maze_size = 6;
        assert_eq!(navigate_maze(&obstacles[..12], maze_size).unwrap(), 22);
    }

    #[test]
    fn part_tow_example() {
        let obstacles = parse_input("input/day18-test.txt").unwrap();
        let maze_size = 6;
        assert_eq!(smallest_break(&obstacles, maze_size), "6,1");
    }
}
