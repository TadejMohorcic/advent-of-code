use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};

pub fn main() -> Result<(), Error> {
    // let path = "input/day09-test.txt";
    let path = "input/day09.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut points = Vec::new();

    let mut x_coords = HashSet::new();
    let mut y_coords = HashSet::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let p: Vec<i64> = line_ok.trim().split(',').map(|x| x.parse().unwrap()).collect();

        x_coords.insert(p[0]);
        y_coords.insert(p[1]);

        let point = Position{x: p[0], y:p[1]};

        points.push(point);
    }

    let mut sorted_x: Vec<i64> = x_coords.iter().map(|x| *x).collect();
    let mut sorted_y: Vec<i64> = y_coords.iter().map(|x| *x).collect();

    sorted_x.sort();
    sorted_y.sort();

    let part_one = largest_area(&points);
    let part_two = largest_valid_area(&points, &sorted_x, &sorted_y);

    println!("--- Day 9: Movie Theater ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position{
    x: i64,
    y: i64
}

fn calculate_area(p1: Position, p2: Position) -> i64 {
    let area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
    area
}

fn largest_area(points: &[Position]) -> i64 {
    let mut largest_area = 0;

    let n = points.len();

    for i in 0..n {
        let p1 = points[i];

        for j in i+1..n {
            let p2 = points[j];

            let area = calculate_area(p1, p2);
            largest_area = max(area, largest_area);
        }
    }

    largest_area
}

fn coordinate_compression(points: &[Position], x_coords: &Vec<i64>, y_coords: &Vec<i64>) -> Vec<Position> {
    let mut new_positions = Vec::new();

    for point in points {
        let new_x = x_coords.iter().position(|n| *n == point.x).unwrap() as i64;
        let new_y = y_coords.iter().position(|n| *n == point.y).unwrap() as i64;

        new_positions.push(Position {x: new_x, y: new_y})
    }

    new_positions
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
                return true
            }
        }
        else {
            let left = min(p1.x, p2.x);
            let right = max(p1.x, p2.x);

            if p.y == p1.y && left <= p.x && p.x <= right {
                return true
            }
        }
    }

    false
}

fn is_inside(p: Position, points: &[Position]) -> bool {
    if is_on_boundary(p, points) {
        return true
    }

    let mut count = 0;
    let n = points.len();

    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];

        if p1.x != p2.x {
            continue;
        }

        let bottom = min(p1.y, p2.y);
        let top = max(p1.y, p2.y);

        if p.x < p1.x && bottom <= p.y && p.y < top {
            count += 1;
        }
    }

    count % 2 == 1
}

fn is_valid_rectangle(p1: Position, p2: Position, points: &[Position], cache: &mut HashMap<Position, bool>) -> bool {
    let min_x = min(p1.x, p2.x);
    let max_x = max(p1.x, p2.x);
    let min_y = min(p1.y, p2.y);
    let max_y = max(p1.y, p2.y);

    for i in min_x..=max_x {
        let p1 = Position{x: i, y: min_y};
        let p2 = Position{x: i, y: max_y};

        let p1_inside: bool;
        let p2_inside: bool;

        if let Some(&is_inside) = cache.get(&p1) {
            p1_inside = is_inside;
        }
        else {
            p1_inside = is_inside(p1, points);
            cache.insert(p1, p1_inside);
        }

        if let Some(&is_inside) = cache.get(&p2) {
            p2_inside = is_inside;
        }
        else {
            p2_inside = is_inside(p2, points);
            cache.insert(p2, p2_inside);
        }

        if !p1_inside || !p2_inside {
            return false
        }
    }

    for i in min_y..=max_y {
        let p1 = Position{x: min_x, y: i};
        let p2 = Position{x: max_x, y: i};

        let p1_inside: bool;
        let p2_inside: bool;

        if let Some(&is_inside) = cache.get(&p1) {
            p1_inside = is_inside;
        }
        else {
            p1_inside = is_inside(p1, points);
            cache.insert(p1, p1_inside);
        }

        if let Some(&is_inside) = cache.get(&p2) {
            p2_inside = is_inside;
        }
        else {
            p2_inside = is_inside(p2, points);
            cache.insert(p2, p2_inside);
        }

        if !p1_inside || !p2_inside {
            return false
        }
    }

    true
}

fn largest_valid_area(points: &[Position], x_coords: &Vec<i64>, y_coords: &Vec<i64>) -> i64 {
    let mut largest_area = 0;

    let compressed_points = coordinate_compression(points, x_coords, y_coords);
    let mut cache = HashMap::new();

    let n = compressed_points.len();

    for i in 0..n {
        let p1 = compressed_points[i];

        for j in i+1..n {
            let p2 = compressed_points[j];

            if p1.x == p2.x || p1.y == p2.y {
                continue;
            }

            let real_p1 = Position {x: x_coords[p1.x as usize], y: y_coords[p1.y as usize]};
            let real_p2 = Position {x: x_coords[p2.x as usize], y: y_coords[p2.y as usize]};

            let area = calculate_area(real_p1, real_p2);

            if area > largest_area && is_valid_rectangle(p1, p2, &compressed_points, &mut cache) {
                largest_area = area;
            }
        }
    }

    largest_area
}

