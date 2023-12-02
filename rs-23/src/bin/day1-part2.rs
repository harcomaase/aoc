use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day1.txt").unwrap();
    let digit_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(|d| d.chars().collect::<Vec<char>>());

    let mut calibration_values = Vec::new();
    for line in input.lines() {
        let mut digits = Vec::with_capacity(10);
        let chars: Vec<char> = line.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            } else if let Some(digit) = is_digit_name(i, &chars, &digit_names) {
                //TODO: can advance by digit_name.len() positions
                digits.push(digit);
            }
        }
        calibration_values.push(*digits.first().unwrap() * 10 + *digits.last().unwrap());
    }
    println!(
        "sum of calibration_values: {}",
        calibration_values.iter().sum::<u32>()
    );
}

fn is_digit_name(i: usize, chars: &[char], digit_names: &[Vec<char>]) -> Option<u32> {
    for (d, digit_name) in digit_names.iter().enumerate() {
        if digit_name.len() > chars.len() - i {
            continue;
        }
        if chars[i..i + digit_name.len()] == digit_name[..] {
            return Some(d as u32 + 1);
        }
    }

    None
}
