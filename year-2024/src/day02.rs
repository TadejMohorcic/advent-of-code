use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day02-test.txt";
    let path = "input/day02.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut reports = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let report: Vec<i64> = line_ok
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        reports.push(report);
    }

    let part_one = check_reports(&reports, false);
    let part_two = check_reports(&reports, true);

    println!("--- Day 2: Red-Nosed Reports ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn check_reports(reports: &[Vec<i64>], part: bool) -> i64 {
    let mut valid_reports = 0;

    for report in reports {
        if is_valid(report) {
            valid_reports += 1;
        } else if part {
            let n = report.len();

            for i in 0..n {
                let new_report = [&report[0..i], &report[i + 1..n]].concat();

                if is_valid(&new_report) {
                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    valid_reports
}

fn is_valid(report: &[i64]) -> bool {
    let n = report.len();
    let mut previous_sign: Option<bool> = None;

    for i in 0..n - 1 {
        let distance = report[i] - report[i + 1];
        let abs_distance = distance.abs();
        let sign = distance > 0;

        if 3 < abs_distance || abs_distance < 1 {
            return false;
        }

        if let Some(prev) = previous_sign {
            if sign != prev {
                return false;
            }
        } else {
            previous_sign = Some(sign);
        }
    }

    true
}
