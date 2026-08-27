use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn main() {
    let (antennas, dim) = parse_input("input/day08.txt");
    let part_one = get_unique_positions(&antennas, dim, false);
    let part_two = get_unique_positions(&antennas, dim, true);

    println!("--- Day 8: Resonant Collinearity ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> (HashMap<char, Vec<(i64, i64)>>, usize) {
    let mut antenna_locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut dimension = 0;

    if let Ok(lines) = crate::read_lines(filename) {
        for (row, line) in lines.map_while(Result::ok).enumerate() {
            for (col, antenna) in line.trim().chars().enumerate().filter(|(_, c)| *c != '.') {
                antenna_locations
                    .entry(antenna)
                    .or_default()
                    .push((row as i64, col as i64));
            }
            dimension = row;
        }
    }

    (antenna_locations, dimension + 1)
}

fn get_antinode_positions(
    antennas: &[(i64, i64)],
    dimension: usize,
    part: bool,
) -> HashSet<(i64, i64)> {
    let mut antinode_positions = HashSet::new();
    let n = antennas.len();

    for i in 0..n {
        let (x1, y1) = antennas[i];

        for (x2, y2) in antennas.iter().take(n).skip(i + 1) {
            let dx = x1 - x2;
            let dy = y1 - y2;

            if part {
                for (mut x, mut y, sign) in [(x1, y1, 1), (*x2, *y2, -1)] {
                    loop {
                        if 0 <= x && (x as usize) < dimension && 0 <= y && (y as usize) < dimension
                        {
                            antinode_positions.insert((x, y));
                        } else {
                            break;
                        }

                        x += sign * dx;
                        y += sign * dy;
                    }
                }
            } else {
                for (x, y, sign) in [(x1, y1, 1), (*x2, *y2, -1)] {
                    let new_x = x + sign * dx;
                    let new_y = y + sign * dy;

                    if 0 <= new_x
                        && (new_x as usize) < dimension
                        && 0 <= new_y
                        && (new_y as usize) < dimension
                    {
                        antinode_positions.insert((new_x, new_y));
                    }
                }
            }
        }
    }

    antinode_positions
}

fn get_unique_positions(
    antennas_map: &HashMap<char, Vec<(i64, i64)>>,
    dimension: usize,
    part: bool,
) -> usize {
    antennas_map
        .values()
        .flat_map(|antennas| get_antinode_positions(antennas, dimension, part))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (antennas, dim) = parse_input("input/day08-test.txt");
        assert_eq!(get_unique_positions(&antennas, dim, false), 14);
    }

    #[test]
    fn part_two_example() {
        let (antennas, dim) = parse_input("input/day08-test.txt");
        assert_eq!(get_unique_positions(&antennas, dim, true), 34);
    }
}
