use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

pub fn main() -> Result<(), Error> {
    // let path = "input/day14-test.txt";
    let path = "input/day14.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut robots = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let (pos, vel) = line_ok.trim().split_once(" ").unwrap();
        let pos = pos.split_once("=").unwrap().1;
        let vel = vel.split_once("=").unwrap().1;

        let pos: Vec<i64> = pos
            .split(',')
            .map(|x| x.parse().expect("Should be a number!"))
            .collect();

        let vel: Vec<i64> = vel
            .split(',')
            .map(|x| x.parse().expect("Should be a number!"))
            .collect();

        let robot = Robot {
            x: pos[0],
            y: pos[1],
            dx: vel[0],
            dy: vel[1],
        };

        robots.push(robot);
    }

    let part_one = move_robots(&robots, 100);
    let part_two = find_christmas_tree(&robots);

    println!("--- Day 14: Restroom Redoubt ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Robot {
    fn move_robot(&self, steps: i64) -> Robot {
        Robot {
            x: (self.x + steps * self.dx).rem_euclid(WIDTH),
            y: (self.y + steps * self.dy).rem_euclid(HEIGHT),
            dx: self.dx,
            dy: self.dy,
        }
    }
}

fn move_robots(robots: &Vec<Robot>, steps: i64) -> usize {
    let mid_width = (WIDTH - 1) / 2;
    let mid_heigh = (HEIGHT - 1) / 2;

    let moved_robots: Vec<Robot> = robots
        .iter()
        .map(|x| x.move_robot(steps))
        .filter(|x| x.x != mid_width || x.y != mid_heigh)
        .collect();

    let quad1: usize = moved_robots
        .iter()
        .filter(|x| x.x > mid_width && x.y < mid_heigh)
        .count();

    let quad2: usize = moved_robots
        .iter()
        .filter(|x| x.x < mid_width && x.y < mid_heigh)
        .count();

    let quad3: usize = moved_robots
        .iter()
        .filter(|x| x.x < mid_width && x.y > mid_heigh)
        .count();

    let quad4: usize = moved_robots
        .iter()
        .filter(|x| x.x > mid_width && x.y > mid_heigh)
        .count();

    quad1 * quad2 * quad3 * quad4
}

fn find_christmas_tree(robots: &Vec<Robot>) -> i64 {
    let mut index = 0;
    let mut unique = robots
        .iter()
        .map(|x| (x.x, x.y))
        .collect::<HashSet<(i64, i64)>>()
        .len();

    for i in 1..=10000 {
        let moved_robots: Vec<Robot> = robots.iter().map(|x| x.move_robot(i)).collect();
        let unique_iter = moved_robots
            .iter()
            .map(|x| (x.x, x.y))
            .collect::<HashSet<(i64, i64)>>()
            .len();

        if unique_iter > unique {
            unique = unique_iter;
            index = i;
        };
    }

    index
}
