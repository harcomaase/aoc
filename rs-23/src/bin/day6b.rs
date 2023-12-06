use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day6.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let time = read_line(lines[0]);
    let record_distance = read_line(lines[1]);

    let mut winning_options = 0;
    for possible_startup_time in 1..time {
        let speed = possible_startup_time;
        let distance = (time - possible_startup_time) * speed;
        if distance > record_distance {
            winning_options += 1;
        }
    }

    println!("{winning_options}");
}

fn read_line(line: &str) -> u64 {
    line.split(':')
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap()
}
