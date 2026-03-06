use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    // let path = "input/day09-test.txt";
    let path = "input/day09.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut input_string: Option<String> = None;

    for line in buffered.lines() {
        let line_ok = line?;

        input_string = Some(line_ok.trim().to_string());
    }

    let (mut files, mut empty_spaces) = get_disk_layout(input_string.unwrap());

    let part_one = format_disk(&files, &empty_spaces);
    let part_two = format_disk_whole(&mut files, &mut empty_spaces);

    println!("--- Day 9: Disk Fragmenter ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn get_disk_layout(disk: String) -> (Vec<(usize, i64)>, Vec<i64>) {
    let files: Vec<(usize, i64)> = disk
        .chars()
        .enumerate()
        .filter(|(e, _)| e % 2 == 0)
        .map(|(e, x)| (e / 2, x.to_digit(10).unwrap() as i64))
        .collect();

    let empty_space: Vec<i64> = disk
        .chars()
        .enumerate()
        .filter(|(e, _)| e % 2 != 0)
        .map(|(_, x)| x.to_digit(10).unwrap() as i64)
        .collect();

    (files, empty_space)
}

fn format_disk(files: &Vec<(usize, i64)>, empty: &Vec<i64>) -> usize {
    let mut checksum = 0;
    let mut global_index = 0;

    let mut local_i = 0;
    let mut local_j = files.len() - 1;
    let mut size_of_j = files[local_j].1;

    loop {
        let value_i = files[local_i].0;

        for _ in 0..files[local_i].1 {
            checksum += value_i * global_index;
            global_index += 1;
        }

        for _ in 0..empty[local_i] {
            if size_of_j == 0 {
                local_j -= 1;
                size_of_j = files[local_j].1;
            }

            checksum += files[local_j].0 * global_index;
            size_of_j -= 1;
            global_index += 1;
        }

        local_i += 1;

        if local_i == local_j {
            if size_of_j > 0 {
                for _ in 0..size_of_j {
                    checksum += files[local_i].0 * global_index;
                    global_index += 1;
                }
            }

            break;
        }
    }

    checksum
}

fn format_disk_whole(files: &mut Vec<(usize, i64)>, empty: &mut Vec<i64>) -> usize {
    let process_order: Vec<(usize, i64)> = files.iter().rev().cloned().collect();

    for file in process_order {
        let file_pos = files.iter().position(|f| f.0 == file.0).unwrap();

        let empty_space = (0..file_pos)
            .find(|&j| empty[j] >= file.1)
            .map(|j| (j, empty[j]));

        if let Some((j, _)) = empty_space {
            if j == file_pos - 1 {
                if !(j == empty.len() - 1) {
                    empty[j + 1] += empty[j];
                }
                empty[j] = 0;
            } else {
                files.remove(file_pos);
                files.insert(j + 1, file);

                if file_pos == empty.len() {
                    empty[j] -= file.1;
                    empty.remove(file_pos - 1);
                    empty.insert(j, 0);
                } else {
                    empty[j] -= file.1;
                    empty[file_pos] += empty[file_pos - 1] + file.1;
                    empty.remove(file_pos - 1);
                    empty.insert(j, 0);
                }
            }
        }
    }

    calculate_checksum(files, empty)
}

fn calculate_checksum(files: &Vec<(usize, i64)>, empty: &Vec<i64>) -> usize {
    let mut checksum = 0;
    let mut global_index = 0;

    for i in 0..files.len() {
        let file = files[i];

        for _ in 0..file.1 {
            checksum += global_index * file.0;
            global_index += 1
        }

        if i != files.len() - 1 {
            global_index += empty[i] as usize;
        }
    }

    checksum
}
