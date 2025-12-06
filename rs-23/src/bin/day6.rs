use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day6.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<u64> = read_line(lines[0]);
    let record_distances: Vec<u64> = read_line(lines[1]);

    let mut winning_options_product = 1;
    for race_no in 0..times.len() {
        let time = times[race_no];
        let record_distance = record_distances[race_no];
        let mut winning_options = 0;
        for possible_startup_time in 1..time {
            let speed = possible_startup_time;
            let distance = (time - possible_startup_time) * speed;
            if distance > record_distance {
                winning_options += 1;
            }
        }
        winning_options_product *= winning_options;
    }

    println!("{winning_options_product}");
}

fn read_line(line: &str) -> Vec<u64> {
    line.split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
