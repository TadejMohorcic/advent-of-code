use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

pub fn main() -> Result<(), Error> {
    // let path = "input/day05-test.txt";
    let path = "input/day05.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut current_event = PageEvent::PageRules;

    let mut page_orders = Vec::new();
    let mut orderings = Vec::new();

    let mut pages = HashSet::new();

    for line in buffered.lines() {
        let line_ok = line?;

        if line_ok.is_empty() {
            current_event = PageEvent::PrintingOrder;
            continue;
        }

        match current_event {
            PageEvent::PageRules => {
                let order: Vec<u64> = line_ok
                    .trim()
                    .split('|')
                    .map(|x| x.parse().unwrap())
                    .collect();

                for page in &order {
                    pages.insert(*page);
                }

                page_orders.push(order.clone());
            }
            PageEvent::PrintingOrder => {
                let ordering: Vec<u64> = line_ok
                    .trim()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect();
                orderings.push(ordering);
            }
        }
    }

    let mut sorted_pages: Vec<u64> = pages.iter().map(|x| *x).collect();
    sorted_pages.sort();

    let (part_one, part_two) = check_order(&page_orders, &orderings, &sorted_pages);

    println!("--- Day 5: Print Queue ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

enum PageEvent {
    PageRules,
    PrintingOrder,
}

fn check_order(rules: &[Vec<u64>], pages: &[Vec<u64>], page_index: &Vec<u64>) -> (u64, u64) {
    let mut part_one = 0;
    let mut part_two = 0;

    let adjecency_matrix = build_adj_matrix(rules, page_index);

    for page in pages {
        if let Some(x) = is_order_valid(page, page_index, &adjecency_matrix) {
            part_one += x;
        } else {
            part_two += bubble_sort(page, page_index, &adjecency_matrix);
        }
    }

    (part_one, part_two)
}

fn build_adj_matrix(rules: &[Vec<u64>], page_index: &Vec<u64>) -> Vec<Vec<u32>> {
    let n = page_index.len();

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; n]; n];

    for rule in rules {
        let before = page_index.iter().position(|n| *n == rule[0]).unwrap() as usize;
        let after = page_index.iter().position(|n| *n == rule[1]).unwrap() as usize;

        matrix[before][after] = 1;
    }

    matrix
}

fn is_order_valid(
    page: &Vec<u64>,
    page_index: &Vec<u64>,
    adj_matrix: &Vec<Vec<u32>>,
) -> Option<u64> {
    let n = page.len();

    for i in 0..n - 1 {
        let before = page_index.iter().position(|n| *n == page[i]).unwrap() as usize;

        for j in i + 1..n {
            let after = page_index.iter().position(|n| *n == page[j]).unwrap() as usize;

            if adj_matrix[before][after] != 1 {
                return None;
            }
        }
    }

    Some(page[n / 2])
}

fn bubble_sort(page: &Vec<u64>, page_index: &Vec<u64>, adj_matrix: &Vec<Vec<u32>>) -> u64 {
    let n = page.len();

    let mut sorted_page = Vec::new();
    sorted_page.push(page[0]);

    for i in 1..n {
        sorted_page.push(page[i]);

        let mut current_index = i;

        loop {
            let current = page_index
                .iter()
                .position(|n| *n == sorted_page[current_index])
                .unwrap() as usize;
            let previous = page_index
                .iter()
                .position(|n| *n == sorted_page[current_index - 1])
                .unwrap() as usize;

            if adj_matrix[previous][current] == 1 {
                break;
            } else {
                sorted_page.swap(current_index, current_index - 1);

                if current_index == 1 {
                    break;
                } else {
                    current_index -= 1;
                }
            }
        }
    }

    sorted_page[n / 2]
}
