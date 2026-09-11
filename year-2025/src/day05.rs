use std::cmp::max;
use std::io;
use std::ops::Range;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (ranges, ingredients) = parse_input("input/day05.txt")?;
    let merged_ranges = merge_ranges(ranges);
    let part_one = count_ingredients(&ingredients, &merged_ranges, false);
    let part_two = count_ingredients(&ingredients, &merged_ranges, true);

    println!("--- Day 5: Cafeteria ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<(Vec<Range<i64>>, Vec<i64>)> {
    let lines = crate::read_lines(filename)?;
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line: Vec<i64> = line
            .trim()
            .split('-')
            .filter_map(|x| x.parse().ok())
            .collect();

        match line.as_slice() {
            [min, max] => ranges.push(*min..*max + 1),
            [i] => ingredients.push(*i),
            _ => continue,
        }
    }

    Ok((ranges, ingredients))
}

fn merge_ranges(mut ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    ranges.sort_by_key(|a| a.start);
    let mut merged_ranges: Vec<Range<i64>> = Vec::new();

    for range in ranges {
        if let Some(prev_range) = merged_ranges.pop() {
            if range.start <= prev_range.end {
                merged_ranges.push(prev_range.start..max(prev_range.end, range.end));
            } else {
                merged_ranges.push(prev_range);
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
}

fn count_ingredients(ingredients: &[i64], ranges: &[Range<i64>], count_all: bool) -> i64 {
    let mut fresh_ingredients = 0;

    if count_all {
        for range in ranges {
            fresh_ingredients += range.end - range.start;
        }
    } else {
        for ingredient in ingredients {
            for range in ranges {
                if range.contains(ingredient) {
                    fresh_ingredients += 1;
                    break;
                }
            }
        }
    }

    fresh_ingredients
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (ranges, ingredients) = parse_input("input/day05-test.txt").unwrap();
        let merged_ranges = merge_ranges(ranges);
        assert_eq!(count_ingredients(&ingredients, &merged_ranges, false), 3);
    }

    #[test]
    fn part_two_example() {
        let (ranges, ingredients) = parse_input("input/day05-test.txt").unwrap();
        let merged_ranges = merge_ranges(ranges);
        assert_eq!(count_ingredients(&ingredients, &merged_ranges, true), 14);
    }
}
