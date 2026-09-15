use std::collections::HashSet;
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (obstacles, boxes, start, instructions) = parse_input("input/day15.txt")?;
    let part_one = move_robot(start, &boxes, &obstacles, &instructions, false);
    let part_two = move_robot(start, &boxes, &obstacles, &instructions, true);

    println!("--- Day 15: Warehouse Woes ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

type Warehouse = (
    HashSet<(i64, i64)>,
    HashSet<(i64, i64)>,
    (i64, i64),
    Vec<char>,
);

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Warehouse> {
    let lines = crate::read_lines(filename)?;
    let mut obstacles = HashSet::new();
    let mut boxes = HashSet::new();
    let mut start = None;
    let mut instructions = Vec::new();
    let mut is_maze = true;

    for (row, line) in lines.map_while(Result::ok).enumerate() {
        if line.is_empty() {
            is_maze = false;
            continue;
        }

        if is_maze {
            obstacles.extend(
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(col, ch)| (ch == '#').then_some((row as i64, col as i64))),
            );
            boxes.extend(
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(col, ch)| (ch == 'O').then_some((row as i64, col as i64))),
            );

            if let Some(col) = line.trim().chars().position(|c| c == '@').map(|n| n as i64) {
                start = Some((row as i64, col));
            }
        } else {
            instructions.extend(line.trim().chars());
        }
    }

    Ok((obstacles, boxes, start.unwrap(), instructions))
}

fn instructions_to_dir(instructions: &[char]) -> Vec<(i64, i64)> {
    instructions
        .iter()
        .map(|ch| match ch {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => (1, 0),
        })
        .collect()
}

fn move_boxes(
    pos: (i64, i64),
    dir: (i64, i64),
    boxes: &mut HashSet<(i64, i64)>,
    obstacles: &HashSet<(i64, i64)>,
) -> bool {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

    if obstacles.contains(&next_pos)
        || (boxes.contains(&next_pos) && !move_boxes(next_pos, dir, boxes, obstacles))
    {
        return false;
    }

    if boxes.contains(&pos) {
        boxes.remove(&pos);
        boxes.insert(next_pos);
    }

    true
}

fn positions_to_check(pos: (i64, i64), dir: (i64, i64)) -> Vec<(i64, i64)> {
    match dir {
        (0, 1) => vec![(pos.0, pos.1 + dir.1)],
        (0, -1) => vec![(pos.0, pos.1 + 2 * dir.1)],
        _ => vec![(pos.0 + dir.0, pos.1), (pos.0 + dir.0, pos.1 - 1)],
    }
}

fn box_positions_to_check(pos: (i64, i64), dir: (i64, i64)) -> Vec<(i64, i64)> {
    match dir {
        (0, _) => vec![(pos.0, pos.1 + 2 * dir.1)],
        _ => {
            vec![
                (pos.0 + dir.0, pos.1),
                (pos.0 + dir.0, pos.1 - 1),
                (pos.0 + dir.0, pos.1 + 1),
            ]
        }
    }
}

fn can_move_wide(
    pos: (i64, i64),
    dir: (i64, i64),
    boxes: &HashSet<(i64, i64)>,
    obstacles: &HashSet<(i64, i64)>,
    is_box: bool,
) -> bool {
    let to_check = if is_box {
        box_positions_to_check(pos, dir)
    } else {
        positions_to_check(pos, dir)
    };

    to_check.iter().all(|p| {
        !obstacles.contains(p)
            && (!boxes.contains(p) || can_move_wide(*p, dir, boxes, obstacles, true))
    })
}

fn move_wide_boxes(
    pos: (i64, i64),
    dir: (i64, i64),
    boxes: &mut HashSet<(i64, i64)>,
    is_box: bool,
) {
    let to_check = if is_box {
        box_positions_to_check(pos, dir)
    } else {
        positions_to_check(pos, dir)
    };

    for p in to_check {
        if boxes.contains(&p) {
            move_wide_boxes(p, dir, boxes, true);
        }
    }

    if is_box {
        boxes.remove(&pos);
        boxes.insert((pos.0 + dir.0, pos.1 + dir.1));
    }
}

fn move_robot(
    mut pos: (i64, i64),
    boxes: &HashSet<(i64, i64)>,
    obstacles: &HashSet<(i64, i64)>,
    instructions: &[char],
    part: bool,
) -> i64 {
    let directions = instructions_to_dir(instructions);

    if part {
        let mut pos = (pos.0, 2 * pos.1);
        let mut boxes = boxes.iter().map(|(row, col)| (*row, col * 2)).collect();
        let obstacles = obstacles.iter().map(|(row, col)| (*row, col * 2)).collect();

        for dir in directions {
            if can_move_wide(pos, dir, &boxes, &obstacles, false) {
                move_wide_boxes(pos, dir, &mut boxes, false);
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
        }

        boxes.iter().map(|(r, c)| 100 * r + c).sum()
    } else {
        let mut boxes = boxes.iter().cloned().collect();

        for dir in directions {
            if move_boxes(pos, dir, &mut boxes, obstacles) {
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
        }

        boxes.iter().map(|(r, c)| 100 * r + c).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (obstacles, boxes, start, instructions) = parse_input("input/day15-test.txt").unwrap();
        assert_eq!(
            move_robot(start, &boxes, &obstacles, &instructions, false),
            10092
        );
    }

    #[test]
    fn part_two_example() {
        let (obstacles, boxes, start, instructions) = parse_input("input/day15-test.txt").unwrap();
        assert_eq!(
            move_robot(start, &boxes, &obstacles, &instructions, true),
            9021
        );
    }
}
