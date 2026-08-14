use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

pub fn main() {
    let paper_rolls = parse_input("input/day04.txt");
    let part_one = remove_paper_rolls(&paper_rolls, false);
    let part_two = remove_paper_rolls(&paper_rolls, true);

    println!("--- Day 4: Printing Department ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

fn parse_input<P: AsRef<Path>>(filename: P) -> HashSet<(i64, i64)> {
    let mut roll_locations = HashSet::new();
    let mut row = 0;

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            roll_locations.extend(
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '@').then(|| (row, i as i64))),
            );
            row += 1;
        }
    }

    roll_locations
}

fn generate_map(towels: &HashSet<(i64, i64)>) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    let mut paper_map = HashMap::new();

    for (x, y) in towels {
        let mut neighbours = Vec::new();
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let candidate = (x + dx, y + dy);
                if towels.contains(&candidate) {
                    neighbours.push(candidate);
                }
            }
        }

        paper_map.insert((*x, *y), neighbours);
    }

    paper_map
}

fn remove_paper_rolls(paper_rolls: &HashSet<(i64, i64)>, repeat: bool) -> usize {
    let paper_map = generate_map(paper_rolls);

    let mut paper_map_len: HashMap<(i64, i64), i64> = paper_map
        .iter()
        .map(|(k, v)| (k.clone(), v.len() as i64))
        .collect();

    let mut to_remove: VecDeque<(i64, i64)> = paper_map_len
        .iter()
        .filter_map(|(k, v)| if *v < 4 { Some(k.clone()) } else { None })
        .collect();

    let mut total_removed = to_remove.len();

    if repeat {
        total_removed = 0;

        while let Some(removed_roll) = to_remove.pop_front() {
            total_removed += 1;

            if let Some(neighbours) = paper_map.get(&removed_roll) {
                for neighbour in neighbours {
                    if let Some(count) = paper_map_len.get_mut(neighbour) {
                        *count -= 1;

                        if *count == 3 {
                            to_remove.push_back(*neighbour);
                        }
                    }
                }
            }
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let paper_rolls = parse_input("input/day04-test.txt");
        assert_eq!(remove_paper_rolls(&paper_rolls, false), 13);
    }

    #[test]
    fn part_two_example() {
        let paper_rolls = parse_input("input/day04-test.txt");
        assert_eq!(remove_paper_rolls(&paper_rolls, true), 43);
    }
}
