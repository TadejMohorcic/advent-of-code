use std::path::Path;

pub fn main() {
    let mut data = parse_input("input/day09.txt");
    let part_one = format_single(&data);
    let part_two = format_whole(&mut data);

    println!("--- Day 9: Disk Fragmenter ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Vec<(i64, i64)> {
    let mut data = Vec::new();

    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            data.extend(line.trim().chars().enumerate().map(|(i, n)| {
                if i % 2 == 0 {
                    ((i / 2) as i64, n.to_digit(10).unwrap() as i64)
                } else {
                    (-1, n.to_digit(10).unwrap() as i64)
                }
            }));
        }
    }

    data
}

fn get_sum(start: i64, end: i64) -> i64 {
    (end - start + 1) * (end + start) / 2
}

fn format_single(data: &[(i64, i64)]) -> i64 {
    let mut checksum = 0;
    let mut global_index = 0;
    let mut i = 0;
    let mut j = data.len() - 1;
    let mut size = data[j].1;

    loop {
        if data[i].0 != -1 {
            checksum += get_sum(global_index, global_index + data[i].1 - 1) * data[i].0;
            global_index += data[i].1;
        } else {
            let mut gap = data[i].1;

            while gap > 0 {
                if size == 0 {
                    j -= 2;
                    size = data[j].1;
                }
                let n = gap.min(size);
                checksum += get_sum(global_index, global_index + n - 1) * data[j].0;
                global_index += n;
                size -= n;
                gap -= n;
            }
        }

        i += 1;

        if i == j {
            if size > 0 {
                checksum += get_sum(global_index, global_index + size - 1) * data[j].0;
            }
            break;
        }
    }

    checksum
}

fn format_whole(data: &mut Vec<(i64, i64)>) -> i64 {
    let process_order: Vec<(i64, i64)> = data.iter().filter(|(id, _)| *id != -1).cloned().collect();

    for file in process_order.into_iter().rev() {
        let file_pos = data.iter().position(|(id, _)| *id == file.0).unwrap();
        let empty = (0..file_pos).find(|&i| data[i].0 == -1 && data[i].1 >= file.1);

        if let Some(i) = empty {
            data[file_pos].0 = -1;

            if file_pos + 1 < data.len() && data[file_pos + 1].0 == -1 {
                data[file_pos].1 += data[file_pos + 1].1;
                data.remove(file_pos + 1);
            }

            if file_pos > 0 && data[file_pos - 1].0 == -1 {
                data[file_pos - 1].1 += data[file_pos].1;
                data.remove(file_pos);
            }

            data[i].1 -= file.1;
            data.insert(i, file);
        }
    }

    calculate_checksum(data)
}

fn calculate_checksum(data: &[(i64, i64)]) -> i64 {
    let mut checksum = 0;
    let mut global_index = 0;

    for (id, len) in data {
        if *id != -1 {
            checksum += get_sum(global_index, global_index + len - 1) * id;
        }
        global_index += len;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = parse_input("input/day09-test.txt");
        assert_eq!(format_single(&data), 1928);
    }

    #[test]
    fn part_two_example() {
        let mut data = parse_input("input/day09-test.txt");
        assert_eq!(format_whole(&mut data), 2858);
    }
}
