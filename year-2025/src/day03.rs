pub fn main() {
    let mut batteries = Vec::new();
    let max_batteries_one = 2;
    let max_batteries_two = 12;

    if let Ok(lines) = crate::read_lines("input/day03.txt") {
        for line in lines.map_while(Result::ok) {
            let battery: Vec<u64> = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            batteries.push(battery);
        }
    }

    let part_one = highest_joltage(&batteries, max_batteries_one);
    let part_two = highest_joltage(&batteries, max_batteries_two);

    println!("--- Day 3: Lobby ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");
}

fn highest_joltage(batteries: &[Vec<u64>], capacity: usize) -> u64 {
    let mut total_joltage = 0;

    for battery in batteries {
        let mut battery_acc = Vec::new();
        let length = battery.len();

        for (i, bat) in battery.iter().enumerate() {
            let remaining_bats = length - i - 1;

            while !battery_acc.is_empty()
                && battery_acc[battery_acc.len() - 1] < bat
                && remaining_bats >= capacity - battery_acc.len()
            {
                battery_acc.pop();
            }

            if battery_acc.len() < capacity {
                battery_acc.push(bat);
            }
        }

        total_joltage += battery_acc.iter().fold(0, |acc, x| acc * 10 + *x);
    }

    total_joltage
}
