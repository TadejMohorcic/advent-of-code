use std::collections::HashSet;
use std::path::Path;

pub fn main() {
    let topographic_map = parse_input("input/day10.txt");
    let part_one = score_all_trailheads(&topographic_map, false);
    let part_two = score_all_trailheads(&topographic_map, true);

    println!("--- Day 10: Hoof It ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<i64>> {
    let mut topographic_map = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            topographic_map.push(
                line.trim()
                    .chars()
                    .map(|n| n.to_digit(10).unwrap() as i64)
                    .collect(),
            );
        }
    }

    topographic_map
}

fn get_trailhead_score(
    start: (i64, i64),
    height: i64,
    topo_map: &[Vec<i64>],
    peaks_found: &mut HashSet<(i64, i64)>,
) -> i64 {
    if height == 9 {
        peaks_found.insert(start);
        return 1;
    }

    let m = topo_map.len();
    let n = topo_map[0].len();
    let mut total_trailheads = 0;

    for i in [-1, 1] {
        let new_x = start.0 + i;
        let new_y = start.1 + i;

        if 0 <= new_x
            && new_x < m as i64
            && topo_map[new_x as usize][start.1 as usize] == height + 1
        {
            total_trailheads +=
                get_trailhead_score((new_x, start.1), height + 1, topo_map, peaks_found);
        }

        if 0 <= new_y
            && new_y < n as i64
            && topo_map[start.0 as usize][new_y as usize] == height + 1
        {
            total_trailheads +=
                get_trailhead_score((start.0, new_y), height + 1, topo_map, peaks_found);
        }
    }

    total_trailheads
}

fn score_all_trailheads(topo_map: &[Vec<i64>], part: bool) -> i64 {
    let mut result = 0;
    for (row_id, row) in topo_map.iter().enumerate() {
        result += row
            .iter()
            .enumerate()
            .filter_map(|(c, t)| {
                (*t == 0).then_some({
                    let mut peaks: HashSet<(i64, i64)> = HashSet::new();
                    let trailhead_score =
                        get_trailhead_score((row_id as i64, c as i64), 0, topo_map, &mut peaks);
                    if part {
                        trailhead_score
                    } else {
                        peaks.len() as i64
                    }
                })
            })
            .sum::<i64>();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let topographic_map = parse_input("input/day10-test.txt");
        assert_eq!(score_all_trailheads(&topographic_map, false), 36);
    }

    #[test]
    fn part_two_example() {
        let topographic_map = parse_input("input/day10-test.txt");
        assert_eq!(score_all_trailheads(&topographic_map, true), 81);
    }
}
