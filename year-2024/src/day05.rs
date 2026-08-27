use std::collections::HashSet;
use std::path::Path;

pub fn main() {
    let (order_rules, pages) = parse_input("input/day05.txt");
    let part_one = get_middle_page(&order_rules, &pages, false);
    let part_two = get_middle_page(&order_rules, &pages, true);

    println!("--- Day 5: Print Queue ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut page_order_rules = Vec::new();
    let mut update_pages = Vec::new();
    let mut is_rule = true;

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                is_rule = false;
                continue;
            }

            if is_rule {
                let rule = line
                    .trim()
                    .split('|')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                page_order_rules.push(rule);
            } else {
                let pages = line
                    .trim()
                    .split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                update_pages.push(pages);
            }
        }
    }

    (page_order_rules, update_pages)
}

fn build_adjacency_matrix(order_rules: &[Vec<i64>]) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut page_numbers = HashSet::new();

    for rule in order_rules {
        page_numbers.insert(rule[0]);
        page_numbers.insert(rule[1]);
    }

    let mut sorted_pages: Vec<i64> = page_numbers.into_iter().collect();
    sorted_pages.sort();
    let n = sorted_pages.len();

    let mut adjacency_matrix = vec![vec![0; n]; n];

    for rule in order_rules {
        let before = sorted_pages.iter().position(|n| *n == rule[0]).unwrap();
        let after = sorted_pages.iter().position(|n| *n == rule[1]).unwrap();

        adjacency_matrix[before][after] = 1;
    }

    (adjacency_matrix, sorted_pages)
}

fn is_page_ordered(
    page: &[i64],
    adjacency_matrix: &[Vec<i64>],
    sorted_pages: &[i64],
) -> Option<i64> {
    let n = page.len();

    for i in 0..n - 1 {
        let before = sorted_pages.iter().position(|n| *n == page[i]).unwrap();
        for p in page.iter().take(n).skip(i + 1) {
            let after = sorted_pages.iter().position(|n| n == p).unwrap();

            if adjacency_matrix[before][after] != 1 {
                return None;
            }
        }
    }

    Some(page[n / 2])
}

fn bubble_sort(page: &[i64], adjacency_matrix: &[Vec<i64>], sorted_pages: &[i64]) -> i64 {
    let mut page_copy = page.to_owned();
    let mut index = 0;
    let n = page.len();

    loop {
        if index == n - 1 {
            break;
        }

        let before = sorted_pages
            .iter()
            .position(|n| *n == page_copy[index])
            .unwrap();
        let after = sorted_pages
            .iter()
            .position(|n| *n == page_copy[index + 1])
            .unwrap();

        if adjacency_matrix[before][after] == 1 {
            index += 1;
        } else {
            page_copy.swap(index, index + 1);
            index = index.saturating_sub(1);
        }
    }

    page_copy[n / 2]
}

fn get_middle_page(order_rules: &[Vec<i64>], pages: &[Vec<i64>], part: bool) -> i64 {
    let (adjacency_matrix, sorted_pages) = build_adjacency_matrix(order_rules);

    pages
        .iter()
        .map(
            |page| match is_page_ordered(page, &adjacency_matrix, &sorted_pages) {
                Some(middle_value) => {
                    if part {
                        0
                    } else {
                        middle_value
                    }
                }
                None => {
                    if part {
                        bubble_sort(page, &adjacency_matrix, &sorted_pages)
                    } else {
                        0
                    }
                }
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (order_rules, pages) = parse_input("input/day05-test.txt");
        assert_eq!(get_middle_page(&order_rules, &pages, false), 143);
    }

    #[test]
    fn part_two_example() {
        let (order_rules, pages) = parse_input("input/day05-test.txt");
        assert_eq!(get_middle_page(&order_rules, &pages, true), 123);
    }
}
