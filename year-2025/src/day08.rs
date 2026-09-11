use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let boxes = parse_input("input/day08.txt")?;
    let (part_one, part_two) = connect_boxes(&boxes, 1000);

    println!("--- Day 8: Playground ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Position>> {
    let lines = crate::read_lines(filename)?;
    let mut boxes = Vec::new();

    for line in lines.map_while(Result::ok) {
        let mut position = line.trim().split(',').map(|x| x.parse::<i64>().unwrap());
        boxes.push(Position {
            x: position.next().unwrap(),
            y: position.next().unwrap(),
            z: position.next().unwrap(),
        })
    }

    Ok(boxes)
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<i64>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut parent_x, mut parent_y) = (self.find(x), self.find(y));

        if parent_x == parent_y {
            false
        } else {
            if self.size[parent_x] < self.size[parent_y] {
                (parent_x, parent_y) = (parent_y, parent_x)
            }

            self.parent[parent_y] = parent_x;
            self.size[parent_x] += self.size[parent_y];
            self.components -= 1;

            true
        }
    }
}

fn squared_distance(p1: Position, p2: Position) -> i64 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)
}

fn get_sorted_distances(positions: &[Position]) -> Vec<(i64, usize, usize)> {
    let n = positions.len();
    let mut distances = Vec::new();

    for i in 0..n {
        let p1 = positions[i];

        for (j, _) in positions.iter().enumerate().take(n).skip(i + 1) {
            let p2 = positions[j];
            let distance = squared_distance(p1, p2);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by_key(|t| t.0);

    distances
}

fn connect_boxes(positions: &[Position], desired_steps: usize) -> (i64, i64) {
    let mut part_one = 1;
    let mut part_two = 1;
    let sorted_distances = get_sorted_distances(positions);
    let mut connected_components = UnionFind::new(positions.len());

    for (steps, (_, i, j)) in sorted_distances.into_iter().enumerate() {
        if steps == desired_steps {
            let mut sorted_sizes = connected_components.size.clone();
            sorted_sizes.sort_by(|a, b| b.cmp(a));
            part_one *= sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2];
        }

        if connected_components.union(i, j) && connected_components.components == 1 {
            part_two *= positions[i].x * positions[j].x;
        }
    }

    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let boxes = parse_input("input/day08-test.txt").unwrap();
        let (part_one, _) = connect_boxes(&boxes, 10);
        assert_eq!(part_one, 40)
    }

    #[test]
    fn part_two_example() {
        let boxes = parse_input("input/day08-test.txt").unwrap();
        let (_, part_two) = connect_boxes(&boxes, 10);
        assert_eq!(part_two, 25272)
    }
}
