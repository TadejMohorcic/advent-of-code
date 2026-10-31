use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (start, end, obstacles) = parse_input("input/day20.txt")?;
    let (small_skip, big_skip, skip_size) = (2, 20, 100);
    let (part_one, part_two) = find_skips(start, end, &obstacles, small_skip, big_skip, skip_size);

    println!("--- Day 20: Race Condition ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

type Racetrack = ((i64, i64), (i64, i64), Vec<Vec<bool>>);

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Racetrack> {
    let lines = crate::read_lines(&filename)?;
    let mut start: Option<(i64, i64)> = None;
    let mut end: Option<(i64, i64)> = None;
    let mut obstacles = Vec::new();

    for (row, line) in lines.map_while(Result::ok).enumerate() {
        let line = line.trim();
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'S' => start = Some((col as i64, row as i64)),
                'E' => end = Some((col as i64, row as i64)),
                _ => {}
            }
        }
        obstacles.push(line.chars().map(|c| c == '#').collect());
    }

    Ok((start.unwrap(), end.unwrap(), obstacles))
}

fn get_racetrack(start: (i64, i64), end: (i64, i64), obstacles: &[Vec<bool>]) -> Vec<(i64, i64)> {
    let mut racetrack = vec![start];
    let (mut prev, mut curr) = (start, start);

    while curr != end {
        let next = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dx, dy)| (curr.0 + dx, curr.1 + dy))
            .find(|&(x, y)| !obstacles[y as usize][x as usize] && (x, y) != prev)
            .unwrap();
        prev = curr;
        curr = next;
        racetrack.push(curr);
    }

    racetrack
}

fn find_skips(
    start: (i64, i64),
    end: (i64, i64),
    obstacles: &[Vec<bool>],
    small_skip: i64,
    big_skip: i64,
    skip_size: i64,
) -> (usize, usize) {
    let racetrack = get_racetrack(start, end, obstacles);
    let mut part_one = 0;
    let mut part_two = 0;

    for i in 0..racetrack.len() {
        for j in (i + skip_size as usize)..racetrack.len() {
            let dist =
                (racetrack[i].0 - racetrack[j].0).abs() + (racetrack[i].1 - racetrack[j].1).abs();
            if dist <= big_skip && (j - i) as i64 - dist >= skip_size {
                part_two += 1;
                if dist <= small_skip {
                    part_one += 1;
                }
            }
        }
    }

    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (start, end, obstacles) = parse_input("input/day20-test.txt").unwrap();
        let (small_skip, big_skip, skip_size) = (2, 20, 1);
        let (part_one, _) = find_skips(start, end, &obstacles, small_skip, big_skip, skip_size);
        assert_eq!(part_one, 44);
    }

    #[test]
    fn part_two_example() {
        let (start, end, obstacles) = parse_input("input/day20-test.txt").unwrap();
        let (small_skip, big_skip, skip_size) = (2, 20, 50);
        let (_, part_two) = find_skips(start, end, &obstacles, small_skip, big_skip, skip_size);
        assert_eq!(part_two, 285);
    }
}
