use std::path::Path;

pub fn main() {
    let word_search = parse_input("input/day04.txt");
    let part_one = find_xmas(&word_search);
    let part_two = find_x_mas(&word_search);

    println!("--- Day 4: Ceres Search ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<char>> {
    let mut word_search = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let chars = line.trim().chars().collect();
            word_search.push(chars);
        }
    }

    word_search
}

fn is_xmas(word_search: &[Vec<char>], i: usize, j: usize) -> i64 {
    let mut count = 0;
    let m = word_search.len();
    let n = word_search[0].len();

    for k in -1i64..2 {
        for l in -1i64..2 {
            if k == 0 && l == 0 {
                continue;
            }
            let end_i = i as i64 + 3 * k;
            let end_j = j as i64 + 3 * l;

            if end_i < 0 || end_i >= m as i64 || end_j < 0 || end_j >= n as i64 {
                continue;
            }

            let i1 = (i as i64 + k) as usize;
            let i2 = (i as i64 + 2 * k) as usize;
            let i3 = (i as i64 + 3 * k) as usize;
            let j1 = (j as i64 + l) as usize;
            let j2 = (j as i64 + 2 * l) as usize;
            let j3 = (j as i64 + 3 * l) as usize;

            count += ((word_search[i1][j1] == 'M')
                && (word_search[i2][j2] == 'A')
                && (word_search[i3][j3] == 'S')) as i64
        }
    }

    count
}

fn find_xmas(word_search: &[Vec<char>]) -> i64 {
    let mut xmas_count = 0;

    for (i, row) in word_search.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'X' {
                xmas_count += is_xmas(word_search, i, j);
            }
        }
    }

    xmas_count
}

fn is_x_mas(word_search: &[Vec<char>], i: usize, j: usize) -> bool {
    let m = word_search.len();
    let n = word_search[0].len();

    if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
        return false;
    }

    let i1 = (i as i64 - 1) as usize;
    let i2 = (i as i64 + 1) as usize;
    let j1 = (j as i64 - 1) as usize;
    let j2 = (j as i64 + 1) as usize;

    let orientation_1 = word_search[i1][j1] == 'M'
        && word_search[i1][j2] == 'M'
        && word_search[i2][j1] == 'S'
        && word_search[i2][j2] == 'S';
    let orientation_2 = word_search[i1][j1] == 'S'
        && word_search[i1][j2] == 'M'
        && word_search[i2][j1] == 'S'
        && word_search[i2][j2] == 'M';
    let orientation_3 = word_search[i1][j1] == 'S'
        && word_search[i1][j2] == 'S'
        && word_search[i2][j1] == 'M'
        && word_search[i2][j2] == 'M';
    let orientation_4 = word_search[i1][j1] == 'M'
        && word_search[i1][j2] == 'S'
        && word_search[i2][j1] == 'M'
        && word_search[i2][j2] == 'S';

    orientation_1 || orientation_2 || orientation_3 || orientation_4
}

fn find_x_mas(word_search: &[Vec<char>]) -> i64 {
    let mut x_mas_count = 0;

    for (i, row) in word_search.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'A' {
                x_mas_count += is_x_mas(word_search, i, j) as i64;
            }
        }
    }

    x_mas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let word_search = parse_input("input/day04-test.txt");
        assert_eq!(find_xmas(&word_search), 18);
    }

    #[test]
    fn part_two_example() {
        let word_search = parse_input("input/day04-test.txt");
        assert_eq!(find_x_mas(&word_search), 9);
    }
}
