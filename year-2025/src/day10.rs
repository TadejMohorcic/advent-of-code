use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use rayon::prelude::*;

pub fn main() -> Result<(), Error> {
    // let path = "input/day10-test.txt";
    let path = "input/day10.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut manuals = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let mut split_line = line_ok.trim().split_whitespace();

        let indicator_lights: Vec<i64> = split_line.next().unwrap().trim_matches(|c| c == '[' || c == ']').chars().map(|c| match c {'#' => 1, _ => 0}).collect();
        let joltage_requirements: Vec<i64> = split_line.next_back().unwrap().trim_matches(|c| c == '{' || c == '}').split(',').map(|c| c.parse().unwrap()).collect();
        let buttons: Vec<Vec<usize>> = split_line.map(|c| c.trim_matches(|x| x == '(' || x == ')').split(',').map(|x| x.parse().unwrap()).collect()).collect();

        let manual = Manual{indicator_lights: indicator_lights, buttons: buttons, joltage_requirements: joltage_requirements};
        manuals.push(manual);
    }

    let part_one = min_button_presses(&manuals);
    let part_two = configure_joltages(&manuals);

    println!("--- Day 10: Factory ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug)]
struct Manual {
    indicator_lights: Vec<i64>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<i64>
}

fn get_combinations(buttons: &[Vec<usize>]) -> Vec<Vec<Vec<usize>>> {
    let n = buttons.len();
    let total = 1 << n;
    let mut power_set = Vec::new();

    for i in 0..total {
        let mut combination = Vec::new();

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                combination.push(buttons[j].clone());
            }
        }

        power_set.push(combination);
    }
    
    power_set
}

fn find_good_combinations<'a>(lights: &[i64], combinations: &'a [Vec<Vec<usize>>]) -> Vec<&'a Vec<Vec<usize>>> {
    let n = lights.len();

    let mut good_combinations = Vec::new();

    for combination in combinations {
        if combination.is_empty() {
            continue;
        }

        let mut new_lights = vec![0; n];

        for button in combination {
            for position in button {
                new_lights[*position] = (new_lights[*position] + 1) % 2;
            }
        }

        if new_lights == *lights {
            good_combinations.push(combination);
        }
    }

    good_combinations
}

fn min_button_presses(manuals: &[Manual]) -> usize {
    manuals.par_iter().map(|manual| {
        let buttons = &manual.buttons;
        let lights = &manual.indicator_lights;
        let combinations = get_combinations(buttons);
        let good_combinations = find_good_combinations(lights, &combinations);
        let mut combinations_len: Vec<usize> = good_combinations.iter().map(|x| x.len()).collect();
        combinations_len.sort();

        combinations_len[0]
    }).sum()
}

fn recursive_joltage<'a>(joltages: &Vec<i64>, combinations: &'a [Vec<Vec<usize>>], cache: &mut HashMap<Vec<i64>, Vec<&'a Vec<Vec<usize>>>>) -> i64 {
    let lights: Vec<i64> = joltages.iter().map(|x| x % 2).collect();

    if lights.iter().all(|x| *x == 0) {
        let new_joltages: Vec<i64> = joltages.iter().map(|x| x / 2).collect();
        
        return 2 * recursive_joltage(&new_joltages, combinations, cache);
    }

    let good_combinations = match cache.entry(lights) {
        Entry::Occupied(e) => e.get().clone(),
        Entry::Vacant(e) => {
            let computed = find_good_combinations(e.key(), combinations);
            e.insert(computed).clone()
        }
    };

    if good_combinations.is_empty() {
        return 10000;
    }

    let mut combinations_to_consider = Vec::new();

    for combination in good_combinations {
        let button_presses: i64;
        let mut copied_joltages = joltages.clone();

        for button in combination {
            for position in button {
                copied_joltages[*position] -= 1;
            }
        }

        let new_joltages: Vec<i64> = copied_joltages.iter().map(|x| x / 2).collect();

        if new_joltages.iter().all(|x| *x == 0) {
            button_presses = combination.len() as i64;
        }
        else if new_joltages.iter().any(|x| *x < 0) {
            button_presses = 10000;
        }
        else {
            button_presses = 2 * recursive_joltage(&new_joltages, combinations, cache) + combination.len() as i64;
        }

        combinations_to_consider.push(button_presses);
    }

    combinations_to_consider.sort();

    combinations_to_consider[0]
}

fn configure_joltages(manuals: &[Manual]) -> i64 {
    manuals.par_iter().map(|manual| {
        let buttons = &manual.buttons;
        let joltages = &manual.joltage_requirements;
        let combinations = get_combinations(buttons);
        let mut cache = HashMap::new();
        let button_presses = recursive_joltage(joltages, &combinations, &mut cache);

        button_presses
    }).sum()
}