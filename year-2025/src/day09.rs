use rustc_hash::FxHashMap;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::path::Path;

pub fn main() {
    let (points, sx, sy) = parse_input("input/day09.txt");
    let (part_one, part_two) = find_largest_area(&points, &sx, &sy);

    println!("--- Day 9: Movie Theater ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

fn parse_input<P: AsRef<Path>>(filename: P) -> (Vec<Position>, Vec<i64>, Vec<i64>) {
    let mut points = Vec::new();
    let mut x_coordinates = HashSet::new();
    let mut y_coordinates = HashSet::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut line = line.trim().split(',').map(|x| x.parse::<i64>().unwrap());
            let x = line.next().unwrap();
            let y = line.next().unwrap();
            points.push(Position { x: x, y: y });
            x_coordinates.insert(x);
            y_coordinates.insert(y);
        }
    }

    let mut sorted_x: Vec<i64> = x_coordinates.iter().map(|x| *x).collect();
    sorted_x.sort();
    let mut sorted_y: Vec<i64> = y_coordinates.iter().map(|x| *x).collect();
    sorted_y.sort();

    (points, sorted_x, sorted_y)
}

fn calculate_area(p1: Position, p2: Position) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

fn coordinate_compression(
    points: &[Position],
    x_coords: &[i64],
    y_coords: &[i64],
) -> Vec<Position> {
    let mut compressed_points = Vec::new();

    for point in points {
        let new_x = x_coords.iter().position(|n| *n == point.x).unwrap() as i64;
        let new_y = y_coords.iter().position(|n| *n == point.y).unwrap() as i64;
        compressed_points.push(Position { x: new_x, y: new_y })
    }

    compressed_points
}

fn is_on_boundary(p: Position, points: &[Position]) -> bool {
    let n = points.len();

    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];

        if p1.x == p2.x {
            let bottom = min(p1.y, p2.y);
            let top = max(p1.y, p2.y);

            if p.x == p1.x && bottom <= p.y && p.y <= top {
                return true;
            }
        } else {
            let left = min(p1.x, p2.x);
            let right = max(p1.x, p2.x);

            if p.y == p1.y && left <= p.x && p.x <= right {
                return true;
            }
        }
    }

    false
}

fn is_inside(p: Position, points: &[Position]) -> bool {
    if is_on_boundary(p, points) {
        return true;
    }

    let mut count = 0;
    let n = points.len();

    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];

        if p1.x != p2.x || p.x >= p1.x {
            continue;
        }

        let bottom = min(p1.y, p2.y);
        let top = max(p1.y, p2.y);

        if bottom <= p.y && p.y < top {
            count += 1;
        }
    }

    count % 2 == 1
}

fn cached_inside(
    pos: Position,
    points: &[Position],
    cache: &mut FxHashMap<Position, bool>,
) -> bool {
    *cache.entry(pos).or_insert_with(|| is_inside(pos, points))
}

fn is_valid_rectangle(
    p1: Position,
    p2: Position,
    points: &[Position],
    cache: &mut FxHashMap<Position, bool>,
) -> bool {
    let left = min(p1.x, p2.x);
    let right = max(p1.x, p2.x);
    let bottom = min(p1.y, p2.y);
    let top = max(p1.y, p2.y);

    for i in left..=right {
        let p1 = Position { x: i, y: bottom };
        let p2 = Position { x: i, y: top };

        if !cached_inside(p1, points, cache) || !cached_inside(p2, points, cache) {
            return false;
        }
    }

    for i in bottom..=top {
        let p1 = Position { x: left, y: i };
        let p2 = Position { x: right, y: i };

        if !cached_inside(p1, points, cache) || !cached_inside(p2, points, cache) {
            return false;
        }
    }

    true
}

fn find_largest_area(points: &[Position], x_coords: &Vec<i64>, y_coords: &Vec<i64>) -> (i64, i64) {
    let mut largest_area = 0;
    let mut largest_valid_area = 0;
    let compressed_points = coordinate_compression(points, x_coords, y_coords);
    let mut cache = FxHashMap::default();
    let n = compressed_points.len();

    for i in 0..n {
        let p1 = compressed_points[i];

        for j in i + 1..n {
            let p2 = compressed_points[j];

            if p1.x == p2.x || p1.y == p2.y {
                continue;
            }

            let real_p1 = Position {
                x: x_coords[p1.x as usize],
                y: y_coords[p1.y as usize],
            };
            let real_p2 = Position {
                x: x_coords[p2.x as usize],
                y: y_coords[p2.y as usize],
            };

            let area = calculate_area(real_p1, real_p2);

            if area > largest_area {
                largest_area = area;
            }

            if area > largest_valid_area
                && is_valid_rectangle(p1, p2, &compressed_points, &mut cache)
            {
                largest_valid_area = area;
            }
        }
    }

    (largest_area, largest_valid_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (points, sx, sy) = parse_input("input/day09-test.txt");
        let (part_one, _) = find_largest_area(&points, &sx, &sy);
        assert_eq!(part_one, 50)
    }

    #[test]
    fn part_two_example() {
        let (points, sx, sy) = parse_input("input/day09-test.txt");
        let (_, part_two) = find_largest_area(&points, &sx, &sy);
        assert_eq!(part_two, 24)
    }
}
