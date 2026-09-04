use std::path::Path;

pub fn main() {
    let (a, b, t) = parse_input("input/day13.txt");
    let part_one = solve_problems(&a, &b, &t, false);
    let part_two = solve_problems(&a, &b, &t, true);

    println!("--- Day 13: Claw Contraption ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

type Problem = (Vec<(i64, i64)>, Vec<(i64, i64)>, Vec<(i64, i64)>);

fn parse_input<P: AsRef<Path>>(filename: P) -> Problem {
    let mut a_buttons = Vec::new();
    let mut b_buttons = Vec::new();
    let mut targets = Vec::new();
    let mut toggle = 0;

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut num_iter = line.trim().split(&['+', ',', '=']);

            if let (Some(x), Some(y)) = (num_iter.nth(1), num_iter.nth(1)) {
                let x = x.parse::<i64>().unwrap();
                let y = y.parse::<i64>().unwrap();
                match toggle {
                    0 => a_buttons.push((x, y)),
                    1 => b_buttons.push((x, y)),
                    _ => targets.push((x, y)),
                }
                toggle = (toggle + 1) % 3;
            }
        }
    }

    (a_buttons, b_buttons, targets)
}

fn solve_problem(a: (i64, i64), b: (i64, i64), mut target: (i64, i64), part: bool) -> i64 {
    if part {
        target.0 += 10_000_000_000_000;
        target.1 += 10_000_000_000_000;
    }

    let det = a.0 * b.1 - a.1 * b.0;
    let m = target.0 * b.1 - target.1 * b.0;
    let n = a.0 * target.1 - a.1 * target.0;

    if det != 0 && m % det == 0 && n % det == 0 {
        return 3 * m / det + n / det;
    }

    0
}

fn solve_problems(
    a_buttons: &[(i64, i64)],
    b_buttons: &[(i64, i64)],
    targets: &[(i64, i64)],
    part: bool,
) -> i64 {
    targets
        .iter()
        .enumerate()
        .map(|(i, t)| solve_problem(a_buttons[i], b_buttons[i], *t, part))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (a, b, t) = parse_input("input/day13-test.txt");
        assert_eq!(solve_problems(&a, &b, &t, false), 480);
    }

    #[test]
    fn part_two_example() {
        let (a, b, t) = parse_input("input/day13-test.txt");
        assert_eq!(solve_problems(&a, &b, &t, true), 875318608908);
    }
}
