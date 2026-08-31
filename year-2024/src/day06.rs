use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

pub fn main() {
    let (rows, cols, guard, size) = parse_input("input/day06.txt");
    let (part_one, to_check) = get_visited_locations(guard, &rows, &cols, size);
    let part_two = get_obstructions(guard, &rows, &cols, &to_check);

    println!("--- Day 6: Guard Gallivant ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

type Grid = HashMap<i64, Arc<Vec<i64>>>;

fn parse_input<P: AsRef<Path>>(filename: P) -> (Grid, Grid, (i64, i64), i64) {
    let mut rows: Grid = HashMap::new();
    let mut cols_build: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut guard_pos = None;
    let mut size = None;

    if let Ok(lines) = crate::read_lines(filename) {
        for (row, line) in lines.map_while(Result::ok).enumerate() {
            size = Some(line.trim().len() as i64);
            let obstacles: Vec<i64> = line
                .trim()
                .chars()
                .enumerate()
                .filter_map(|(n, c)| (c == '#').then_some(n as i64))
                .collect();

            if let Some(col) = line.trim().chars().position(|c| c == '^').map(|n| n as i64) {
                guard_pos = Some((row as i64, col));
            }

            if obstacles.is_empty() {
                continue;
            }

            for c in &obstacles {
                cols_build.entry(*c).or_default().push(row as i64);
            }

            rows.insert(row as i64, Arc::new(obstacles));
        }
    }

    let cols: Grid = cols_build
        .into_iter()
        .map(|(k, v)| (k, Arc::new(v)))
        .collect();

    (rows, cols, guard_pos.unwrap(), size.unwrap())
}

fn get_next_obstacle(
    search_space: Option<&Arc<Vec<i64>>>,
    position: i64,
    is_increasing: bool,
) -> Option<i64> {
    match search_space {
        Some(obstacles) => {
            if is_increasing {
                let (mut left, mut right) = (0, obstacles.len());

                while left < right {
                    let mid = left + (right - left) / 2;

                    if obstacles[mid] > position {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
                if left < obstacles.len() {
                    Some(obstacles[left] - 1)
                } else {
                    None
                }
            } else {
                let (mut left, mut right) = (0, obstacles.len());

                while left < right {
                    let mid = left + (right - left) / 2;
                    if obstacles[mid] < position {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                if left > 0 {
                    Some(obstacles[left - 1] + 1)
                } else {
                    None
                }
            }
        }
        None => None,
    }
}

fn get_visited_locations(
    start: (i64, i64),
    rows: &Grid,
    cols: &Grid,
    grid_size: i64,
) -> (usize, HashSet<(i64, i64)>) {
    let mut visited_locations = HashSet::new();
    let mut location = start;
    let mut direction = (-1, 0);

    loop {
        let (x, y) = location;
        let (dx, dy) = direction;
        let dir = if dx != 0 { dx } else { dy };
        let (pos, search) = if dx == dir {
            (x, cols.get(&y))
        } else {
            (y, rows.get(&x))
        };

        if let Some(next) = get_next_obstacle(search, pos, dir > 0) {
            let min = min(pos, next);
            let max = max(pos, next);

            for i in min..=max {
                if dir == dx {
                    visited_locations.insert((i, y));
                } else {
                    visited_locations.insert((x, i));
                }
            }

            location = if dir == dx { (next, y) } else { (x, next) };
            direction = (direction.1, -direction.0);
        } else {
            if dir > 0 {
                for i in pos..grid_size {
                    if dir == dx {
                        visited_locations.insert((i, y));
                    } else {
                        visited_locations.insert((x, i));
                    }
                }
            } else {
                for i in 0..=pos {
                    if dir == dx {
                        visited_locations.insert((i, y));
                    } else {
                        visited_locations.insert((x, i));
                    }
                }
            }
            break;
        }
    }

    (visited_locations.len(), visited_locations)
}

fn is_looping(start: (i64, i64), rows: &Grid, cols: &Grid) -> bool {
    let mut visited_locations = HashSet::new();
    let mut location = start;
    let mut direction = (-1, 0);

    loop {
        let (x, y) = location;
        let (dx, dy) = direction;

        if visited_locations.contains(&(x, y, dx, dy)) {
            return true;
        }

        let dir = if dx != 0 { dx } else { dy };
        let (pos, search) = if dx == dir {
            (x, cols.get(&y))
        } else {
            (y, rows.get(&x))
        };

        if let Some(next) = get_next_obstacle(search, pos, dir > 0) {
            let min = min(pos, next);
            let max = max(pos, next);

            for i in min..=max {
                if dir == dx {
                    visited_locations.insert((i, y, dx, dy));
                } else {
                    visited_locations.insert((x, i, dx, dy));
                }
            }

            location = if dir == dx { (next, y) } else { (x, next) };
            direction = (direction.1, -direction.0);
        } else {
            break;
        }
    }

    false
}

fn get_obstructions(
    start: (i64, i64),
    rows: &Grid,
    cols: &Grid,
    to_check: &HashSet<(i64, i64)>,
) -> i64 {
    to_check
        .par_iter()
        .map(|(x, y)| {
            let mut new_rows = rows.clone();
            let mut new_cols = cols.clone();

            let row = new_rows.entry(*x).or_insert_with(|| Arc::new(Vec::new()));
            let mut row_vec = (**row).clone();
            let r = row_vec.binary_search(y).unwrap_or_else(|n| n);
            row_vec.insert(r, *y);
            *row = Arc::new(row_vec);

            let col = new_cols.entry(*y).or_insert_with(|| Arc::new(Vec::new()));
            let mut col_vec = (**col).clone();
            let c = col_vec.binary_search(x).unwrap_or_else(|n| n);
            col_vec.insert(c, *x);
            *col = Arc::new(col_vec);

            is_looping(start, &new_rows, &new_cols) as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (rows, cols, guard, size) = parse_input("input/day06-test.txt");
        let (visited_len, _) = get_visited_locations(guard, &rows, &cols, size);
        assert_eq!(visited_len, 41);
    }

    #[test]
    fn part_two_example() {
        let (rows, cols, guard, size) = parse_input("input/day06-test.txt");
        let (_, to_check) = get_visited_locations(guard, &rows, &cols, size);
        assert_eq!(get_obstructions(guard, &rows, &cols, &to_check), 6);
    }
}
