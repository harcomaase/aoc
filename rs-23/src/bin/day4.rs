use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day4.txt").unwrap();

    let mut points = 0;
    for line in input.lines() {
        let mut all_numbers = line.split(':').nth(1).unwrap().trim().split('|');
        let winning_numbers = extract_numbers(all_numbers.next().unwrap());
        let player_numbers = extract_numbers(all_numbers.next().unwrap());
        let pow = player_numbers.iter().fold(0, |acc, v| {
            if winning_numbers.contains(v) {
                acc + 1
            } else {
                acc
            }
        });
        if pow > 0 {
            points += 2i32.pow(pow - 1);
        }
    }
    println!("points: {points}");
}

fn extract_numbers(str: &str) -> Vec<u32> {
    str.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}
