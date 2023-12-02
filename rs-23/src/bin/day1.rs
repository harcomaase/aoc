use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day1.txt").unwrap();

    let mut calibration_values = Vec::new();
    for line in input.lines() {
        let mut digits = Vec::with_capacity(10);
        for c in line.chars() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            }
        }
        calibration_values.push(*digits.first().unwrap() * 10 + *digits.last().unwrap());
    }
    println!(
        "sum of calibration_values: {}",
        calibration_values.iter().sum::<u32>()
    );
}
