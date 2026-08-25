use std::path::Path;

pub fn main() {
    let (shapes, regions) = parse_input("input/day12.txt");
    let part_one = get_possible_regions(&shapes, &regions);

    println!("--- Day 12: Christmas Tree Farm ---");
    println!(" - Part one solution: {}\n", part_one);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> (Vec<i64>, Vec<(i64, Vec<i64>)>) {
    let mut shapes = Vec::new();
    let mut shape_size = 0;
    let mut regions = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            match line.trim().split_once(": ") {
                Some(split) => {
                    let area = split
                        .0
                        .split("x")
                        .map(|n| n.parse::<i64>().unwrap())
                        .product();
                    let shapes_used = split
                        .1
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect();
                    regions.push((area, shapes_used))
                }
                None => {
                    if line.is_empty() {
                        shapes.push(shape_size);
                        shape_size = 0;
                        continue;
                    }

                    shape_size += line.trim().chars().filter(|c| *c == '#').count() as i64;
                }
            }
        }
    }

    (shapes, regions)
}

fn get_possible_regions(shapes: &[i64], regions: &[(i64, Vec<i64>)]) -> usize {
    regions
        .iter()
        .filter_map(|(size, shapes_used)| {
            let mut needed_area = 0;
            for (i, s) in shapes_used.iter().enumerate() {
                if *s == 0 {
                    continue;
                }
                needed_area += s * shapes[i];
            }
            (needed_area <= *size).then_some(())
        })
        .count()
}
