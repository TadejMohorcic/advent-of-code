use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (locks, keys) = parse_input("input/day25.txt")?;
    let part_one = check_locks_and_keys(&locks, &keys);

    println!("--- Day 25: Code Chronicle ---");
    println!(" - Part one solution: {}\n", part_one);

    Ok(())
}

type LocksAndKeys = (Vec<Vec<i64>>, Vec<Vec<i64>>);

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<LocksAndKeys> {
    let lines = crate::read_lines(&filename)?;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut is_lock = None;
    let mut current = vec![0; 5];

    for line in lines.map_while(Result::ok) {
        let pins: Vec<i64> = line
            .trim()
            .chars()
            .map(|ch| if ch == '#' { 1 } else { 0 })
            .collect();

        if pins.is_empty() {
            match is_lock {
                Some(true) => locks.push(current),
                Some(false) => keys.push(current),
                None => unreachable!("Is either a key or a lock!"),
            }

            current = vec![0; 5];
            is_lock = None;
            continue;
        }

        current = current.iter().zip(&pins).map(|(a, b)| a + b).collect();

        if is_lock.is_none() {
            is_lock = Some(pins.iter().sum::<i64>() as usize == pins.len());
        }
    }

    if current.iter().sum::<i64>() != 0 {
        match is_lock {
            Some(true) => locks.push(current),
            Some(false) => keys.push(current),
            None => unreachable!("Is either a key or a lock!"),
        }
    }

    Ok((locks, keys))
}

fn check_locks_and_keys(locks: &[Vec<i64>], keys: &[Vec<i64>]) -> i64 {
    let mut valid_pairs = 0;

    for lock in locks {
        for key in keys {
            valid_pairs += lock.iter().zip(key).map(|(l, k)| l + k).all(|x| x < 8) as i64;
        }
    }

    valid_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (locks, keys) = parse_input("input/day25-test.txt").unwrap();
        assert_eq!(check_locks_and_keys(&locks, &keys), 3);
    }
}
