use std::path::Path;

pub fn main() {
    let reports = parse_input("input/day02.txt");
    let part_one = check_reports(&reports, false);
    let part_two = check_reports(&reports, true);

    println!("--- Day 2: Red-Nosed Reports ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<i64>> {
    let mut reports = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            reports.push(
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        }
    }

    reports
}

fn get_differences(report: &[i64]) -> Vec<i64> {
    report.array_windows::<2>().map(|[x, y]| y - x).collect()
}

fn is_valid_report(report: &[i64]) -> bool {
    let differences = get_differences(report);

    if differences.iter().all(|n| *n > 0 && *n < 4) || differences.iter().all(|n| *n < 0 && *n > -4)
    {
        return true;
    }

    false
}

fn check_reports(reports: &[Vec<i64>], part: bool) -> usize {
    reports
        .iter()
        .filter_map(|report| {
            if is_valid_report(report) {
                Some(())
            } else if part {
                let n = report.len();

                for i in 0..n {
                    let new_report = [&report[0..i], &report[i + 1..n]].concat();

                    if is_valid_report(&new_report) {
                        return Some(());
                    }
                }

                None
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let reports = parse_input("input/day02-test.txt");
        assert_eq!(check_reports(&reports, false), 2);
    }

    #[test]
    fn part_two_example() {
        let reports = parse_input("input/day02-test.txt");
        assert_eq!(check_reports(&reports, true), 4);
    }
}
