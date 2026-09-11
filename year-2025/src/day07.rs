use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let splitters = parse_input("input/day07.txt")?;
    let (part_one, part_two) = move_down_the_manifold(&splitters);

    println!("--- Day 7: Laboratories ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<i64>>> {
    let lines = crate::read_lines(filename)?;
    let mut splitters = Vec::new();

    for line in lines.map_while(Result::ok) {
        let row: Vec<i64> = line
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(i, x)| (x == '^').then_some(i as i64))
            .collect();

        if !row.is_empty() {
            splitters.push(row);
        }
    }

    Ok(splitters)
}

fn move_down_the_manifold(locations: &[Vec<i64>]) -> (i64, i64) {
    let mut splits = 0;
    let starting_pos = locations[0][0];
    let mut beams = vec![0; 2 * (starting_pos as usize) + 1];
    beams[starting_pos as usize] = 1;

    for loc in locations {
        let mut new_beams = vec![0; 2 * (starting_pos as usize) + 1];
        let beam_ids: Vec<usize> = beams
            .iter()
            .enumerate()
            .filter_map(|(i, x)| (*x > 0).then_some(i))
            .collect();

        for b in beam_ids {
            if loc.contains(&(b as i64)) {
                new_beams[b - 1] += beams[b];
                new_beams[b + 1] += beams[b];
                new_beams[b] = 0;
                splits += 1;
            } else {
                new_beams[b] += beams[b];
            }
        }

        beams = new_beams;
    }

    (splits, beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let splitters = parse_input("input/day07-test.txt").unwrap();
        let (part_one, _) = move_down_the_manifold(&splitters);
        assert_eq!(part_one, 21);
    }

    #[test]
    fn part_two_example() {
        let splitters = parse_input("input/day07-test.txt").unwrap();
        let (_, part_two) = move_down_the_manifold(&splitters);
        assert_eq!(part_two, 40);
    }
}
