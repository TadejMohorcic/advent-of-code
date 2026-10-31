use std::collections::HashMap;
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (patterns, towels) = parse_input("input/day19.txt")?;
    let (part_one, part_two) = check_designs(&towels, &patterns);

    println!("--- Day 19: Linen Layout ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<(Vec<String>, Vec<String>)> {
    let lines = crate::read_lines(&filename)?;
    let mut is_towel = false;
    let mut patterns: Option<Vec<String>> = None;
    let mut towels = Vec::new();

    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            is_towel = true;
            continue;
        }

        if is_towel {
            towels.push(line.trim().to_string());
        } else {
            patterns = Some(line.trim().split(", ").map(|x| x.to_string()).collect());
        }
    }

    Ok((patterns.unwrap(), towels))
}

fn check_design(
    towel: &String,
    patterns: &[String],
    index: usize,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    let n = towel.len();

    if index == n {
        1
    } else {
        if let Some(res) = cache.get(&towel[index..].to_string()) {
            *res
        } else {
            let mut result = 0;

            for pattern in patterns {
                let m = pattern.len();

                if index + m <= n && &towel[index..index + m] == pattern {
                    result += check_design(towel, patterns, index + m, cache);
                }
            }

            cache.insert(towel[index..].to_string(), result);

            result
        }
    }
}

fn check_designs(towels: &[String], patterns: &[String]) -> (i64, i64) {
    let mut part_one = 0;
    let mut part_two = 0;
    let mut cache = HashMap::new();

    towels.iter().for_each(|towel| {
        let designs_avalible = check_design(towel, patterns, 0, &mut cache);

        part_one += (designs_avalible > 0) as i64;
        part_two += designs_avalible;
    });

    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (patterns, towels) = parse_input("input/day19-test.txt").unwrap();
        let (part_one, _) = check_designs(&towels, &patterns);
        assert_eq!(part_one, 6);
    }

    #[test]
    fn part_two_example() {
        let (patterns, towels) = parse_input("input/day19-test.txt").unwrap();
        let (_, part_two) = check_designs(&towels, &patterns);
        assert_eq!(part_two, 16);
    }
}
