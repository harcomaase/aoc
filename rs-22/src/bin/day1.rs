use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day1.txt").unwrap();

    let mut totals = Vec::new();
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            totals.push(total);
            total = 0;
            continue;
        }
        let value = line.parse::<i32>().unwrap();
        total += value;
    }

    totals.sort_by(|a, b| b.cmp(a));

    println!("max: {}", totals[0]);
}
