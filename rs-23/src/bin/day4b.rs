use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day4.txt").unwrap();
    let input_lines: Vec<&str> = input.lines().collect();
    let mut card_quantity = vec![1; input_lines.len()];
    for (i, line) in input_lines.iter().enumerate() {
        let mut all_numbers = line.split(':').nth(1).unwrap().trim().split('|');
        let winning_numbers = extract_numbers(all_numbers.next().unwrap());
        let player_numbers = extract_numbers(all_numbers.next().unwrap());
        let matches: usize = player_numbers
            .iter()
            .map(|n| if winning_numbers.contains(n) { 1 } else { 0 })
            .sum();
        let current_card_quantity = card_quantity[i];
        for j in i + 1..=i + matches {
            card_quantity[j] += current_card_quantity;
        }
    }
    println!("total: {}", card_quantity.iter().sum::<i32>());
}

fn extract_numbers(str: &str) -> Vec<u32> {
    str.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}
