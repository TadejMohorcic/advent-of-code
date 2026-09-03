use std::collections::HashMap;
use std::path::Path;

pub fn main() {
    let stone_map = parse_input("input/day11.txt");
    let part_one = blink(&stone_map, 25);
    let part_two = blink(&stone_map, 75);

    println!("--- Day 11: Plutonian Pebbles ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> HashMap<i64, i64> {
    let mut stone_map: HashMap<i64, i64> = HashMap::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let stones = line.split_whitespace();
            for stone in stones {
                *stone_map.entry(stone.parse::<i64>().unwrap()).or_default() += 1;
            }
        }
    }

    stone_map
}

fn blink(stones: &HashMap<i64, i64>, times: usize) -> i64 {
    let mut stones = stones.to_owned();

    for _ in 0..times {
        let mut new_stones = HashMap::new();

        for (stone, count) in &stones {
            match stone {
                0 => *new_stones.entry(1).or_default() += count,
                x if (x.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let modulo = 10_u64.pow(x.checked_ilog10().unwrap_or(0).div_ceil(2)) as i64;
                    let right_num = x % modulo;
                    let left_num = x / modulo;
                    *new_stones.entry(left_num).or_default() += count;
                    *new_stones.entry(right_num).or_default() += count;
                }
                _ => *new_stones.entry(stone * 2024).or_default() += count,
            }
        }

        stones = new_stones;
    }

    stones.iter().fold(0, |acc, (_, c)| acc + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let stone_map = parse_input("input/day11-test.txt");
        assert_eq!(blink(&stone_map, 25), 55312);
    }

    #[test]
    fn part_two_example() {
        let stone_map = parse_input("input/day11-test.txt");
        assert_eq!(blink(&stone_map, 75), 65601038650482);
    }
}
