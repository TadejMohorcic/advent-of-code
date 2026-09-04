use std::collections::HashSet;
use std::path::Path;

pub fn main() {
    let farm_map = parse_input("input/day12.txt");
    let (part_one, part_two) = calculate_price(&farm_map);

    println!("--- Day 12: Garden Groups ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<Vec<char>> {
    let mut farm_map = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            farm_map.push(line.trim().chars().collect());
        }
    }

    farm_map
}

fn flood_fill(
    row: usize,
    col: usize,
    map: &Vec<Vec<char>>,
    char: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (i64, i64, i64) {
    let mut area = 1;
    let mut border = 0;
    let mut sides = 0;
    let r_i64 = row as i64;
    let c_i64 = col as i64;

    visited.insert((row, col));

    let in_shape = |r: i64, c: i64| -> bool {
        r >= 0
            && c >= 0
            && map.len() > r as usize
            && map[0].len() > c as usize
            && map[r as usize][c as usize] == char
    };

    sides += (!in_shape(r_i64 - 1, c_i64)
        && (!in_shape(r_i64, c_i64 - 1) || in_shape(r_i64 - 1, c_i64 - 1))) as i64;
    sides += (!in_shape(r_i64 + 1, c_i64)
        && (!in_shape(r_i64, c_i64 - 1) || in_shape(r_i64 + 1, c_i64 - 1))) as i64;
    sides += (!in_shape(r_i64, c_i64 - 1)
        && (!in_shape(r_i64 - 1, c_i64) || in_shape(r_i64 - 1, c_i64 - 1))) as i64;
    sides += (!in_shape(r_i64, c_i64 + 1)
        && (!in_shape(r_i64 - 1, c_i64) || in_shape(r_i64 - 1, c_i64 + 1))) as i64;

    for dir in [-1, 1] {
        let new_row = r_i64 + dir;
        let new_col = c_i64 + dir;

        if in_shape(new_row, c_i64) {
            if !visited.contains(&(new_row as usize, c_i64 as usize)) {
                let (a, b, s) = flood_fill(new_row as usize, col, map, char, visited);
                area += a;
                border += b;
                sides += s;
            }
        } else {
            border += 1;
        }

        if in_shape(r_i64, new_col) {
            if !visited.contains(&(row, new_col as usize)) {
                let (a, b, s) = flood_fill(row, new_col as usize, map, char, visited);
                area += a;
                border += b;
                sides += s;
            }
        } else {
            border += 1;
        }
    }

    (area, border, sides)
}

fn calculate_price(map: &Vec<Vec<char>>) -> (i64, i64) {
    let mut price_border = 0;
    let mut price_sides = 0;
    let mut visited = HashSet::new();

    for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if !visited.contains(&(i, j)) {
                let (area, border, sides) = flood_fill(i, j, map, *char, &mut visited);
                price_border += area * border;
                price_sides += area * sides;
            }
        }
    }

    (price_border, price_sides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let farm_map = parse_input("input/day12-test.txt");
        let (part_one, _) = calculate_price(&farm_map);
        assert_eq!(part_one, 1930);
    }

    #[test]
    fn part_two_example() {
        let farm_map = parse_input("input/day12-test.txt");
        let (_, part_two) = calculate_price(&farm_map);
        assert_eq!(part_two, 1206);
    }
}
