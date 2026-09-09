use rayon::prelude::*;
use std::collections::HashSet;
use std::path::Path;

pub fn main() {
    let steps = 100;
    let width = 101;
    let height = 103;
    let robots = parse_input("input/day14.txt");
    let part_one = get_safety_factor(&robots, steps, width, height);
    let part_two = find_christmas_tree(&robots, width, height);

    println!("--- Day 14: Restroom Redoubt ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<i64>> {
    let mut robots = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            robots.push(
                line.split(&['=', ',', ' '][..])
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect(),
            );
        }
    }

    robots
}

fn move_robots(robots: &[Vec<i64>], steps: i64, w: i64, h: i64) -> Vec<Vec<i64>> {
    robots
        .par_iter()
        .map(|r| {
            vec![
                (r[0] + steps * r[2]).rem_euclid(w),
                (r[1] + steps * r[3]).rem_euclid(h),
                r[2],
                r[3],
            ]
        })
        .collect()
}

fn get_safety_factor(robots: &[Vec<i64>], steps: i64, w: i64, h: i64) -> usize {
    let moved_robots = move_robots(robots, steps, w, h);
    let mid_w = (w - 1) / 2;
    let mid_h = (h - 1) / 2;

    let q1 = moved_robots
        .iter()
        .filter(|r| r[0] < mid_w && r[1] < mid_h)
        .count();
    let q2 = moved_robots
        .iter()
        .filter(|r| r[0] > mid_w && r[1] < mid_h)
        .count();
    let q3 = moved_robots
        .iter()
        .filter(|r| r[0] < mid_w && r[1] > mid_h)
        .count();
    let q4 = moved_robots
        .iter()
        .filter(|r| r[0] > mid_w && r[1] > mid_h)
        .count();

    q1 * q2 * q3 * q4
}

fn xgcd(a: i64, b: i64) -> (i64, i64) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;

    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s;
        t0 = t1;
        t1 = t;
    }

    (s0, t0)
}

fn find_christmas_tree(robots: &[Vec<i64>], w: i64, h: i64) -> i64 {
    let mut a1 = 0;
    let mut a2 = 0;
    let mut x_count = w as usize;
    let mut y_count = h as usize;
    let (m1, m2) = xgcd(w, h);

    for i in 0..w.max(h) {
        let moved_robots = move_robots(robots, i, w, h);
        let cx = moved_robots
            .iter()
            .map(|r| r[0])
            .collect::<HashSet<i64>>()
            .len();
        let cy = moved_robots
            .iter()
            .map(|r| r[1])
            .collect::<HashSet<i64>>()
            .len();

        if cx < x_count {
            x_count = cx;
            a1 = i;
        }

        if cy < y_count {
            y_count = cy;
            a2 = i;
        }
    }

    (a1 * m2 * h + a2 * m1 * w) % (w * h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let robots = parse_input("input/day14-test.txt");
        assert_eq!(get_safety_factor(&robots, 100, 11, 7), 12);
    }

    #[test]
    fn part_two_example() {
        let robots = parse_input("input/day14-test.txt");
        assert_eq!(find_christmas_tree(&robots, 11, 7), -63);
    }
}
