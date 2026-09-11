use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let manuals = parse_input("input/day10.txt")?;
    let part_one = toggle_lights(&manuals);
    let part_two = toggle_joltages(&manuals);

    println!("--- Day 10: Factory ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

struct Manual {
    indicator_light_diagram: Vec<i64>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<i64>,
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Manual>> {
    let lines = crate::read_lines(filename)?;
    let mut manuals = Vec::new();

    for line in lines.map_while(Result::ok) {
        let mut line = line.split_whitespace();
        let indicator_light_diagram = line
            .next()
            .unwrap()
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| match c {
                '#' => 1,
                _ => 0,
            })
            .collect();
        let joltage_requirements = line
            .next_back()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let buttons = line
            .map(|c| {
                c.trim_matches(['(', ')'])
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();
        manuals.push(Manual {
            indicator_light_diagram,
            joltage_requirements,
            buttons,
        });
    }

    Ok(manuals)
}

fn get_all_combinations(n: usize) -> Vec<Vec<usize>> {
    let mut all_combinations = Vec::new();
    let total = 1 << n;

    for i in 0..total {
        let mut combination = Vec::new();

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                combination.push(j);
            }
        }

        all_combinations.push(combination);
    }

    all_combinations
}

fn get_valid_combinations(
    lights: &[i64],
    buttons: &[Vec<usize>],
    combinations: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut valid_combinations = Vec::new();
    let m = lights.len();

    for combination in combinations {
        if combination.is_empty() {
            continue;
        }

        let mut test_lights = vec![0; m];

        for button_id in combination {
            let button = &buttons[*button_id];

            for toggle in button {
                test_lights[*toggle] = (test_lights[*toggle] + 1) % 2;
            }
        }

        if test_lights == *lights {
            valid_combinations.push(combination.clone());
        }
    }

    valid_combinations
}

fn toggle_lights(manuals: &[Manual]) -> usize {
    manuals
        .par_iter()
        .map(|manual| {
            let lights = &manual.indicator_light_diagram;
            let buttons = &manual.buttons;
            let all_combinations = get_all_combinations(buttons.len());
            let valid_combinations = get_valid_combinations(lights, buttons, &all_combinations);
            let mut combination_lengths: Vec<usize> =
                valid_combinations.iter().map(|combo| combo.len()).collect();
            combination_lengths.sort();

            combination_lengths[0]
        })
        .sum()
}

fn configure_joltages_recursive(
    joltages: &[i64],
    buttons: &[Vec<usize>],
    combinations: &[Vec<usize>],
    cache: &mut FxHashMap<Vec<i64>, Vec<Vec<usize>>>,
) -> usize {
    let lights: Vec<i64> = joltages.iter().map(|n| n % 2).collect();

    if lights.iter().all(|n| *n == 0) {
        let new_joltages: Vec<i64> = joltages.iter().map(|n| n / 2).collect();
        return 2 * configure_joltages_recursive(&new_joltages, buttons, combinations, cache);
    }

    let good_combinations = cache
        .entry(lights)
        .or_insert_with_key(|lights| get_valid_combinations(lights, buttons, combinations))
        .clone();

    if good_combinations.is_empty() {
        return 10000;
    }

    let mut combinations_length = Vec::new();

    for combination in good_combinations {
        let mut joltages_copy = joltages.to_owned();

        for button_id in &combination {
            let button = &buttons[*button_id];

            for toggle in button {
                joltages_copy[*toggle] -= 1;
            }
        }

        let new_joltages: Vec<i64> = joltages_copy.iter().map(|n| n / 2).collect();

        if new_joltages.iter().all(|n| *n == 0) {
            combinations_length.push(combination.len());
        } else if new_joltages.iter().any(|n| *n < 0) {
            combinations_length.push(10000);
        } else {
            combinations_length.push(
                2 * configure_joltages_recursive(&new_joltages, buttons, combinations, cache)
                    + combination.len(),
            )
        }
    }

    combinations_length.sort();

    combinations_length[0]
}

fn toggle_joltages(manuals: &[Manual]) -> usize {
    manuals
        .par_iter()
        .map(|manual| {
            let buttons = &manual.buttons;
            let joltages = &manual.joltage_requirements;
            let all_combinations = get_all_combinations(buttons.len());
            let mut cache = FxHashMap::default();

            configure_joltages_recursive(joltages, buttons, &all_combinations, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let manuals = parse_input("input/day10-test.txt").unwrap();
        assert_eq!(toggle_lights(&manuals), 7)
    }

    #[test]
    fn part_two_example() {
        let manuals = parse_input("input/day10-test.txt").unwrap();
        assert_eq!(toggle_joltages(&manuals), 33)
    }
}
