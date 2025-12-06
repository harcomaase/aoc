use std::fs;

fn main() {
    let filename = "../input/19/day1.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut required_fuel = 0;
    for line in file.lines() {
        if line.is_empty() {
            continue;
        }
        let mass = line.parse::<u32>().unwrap();
        required_fuel += (mass / 3) - 2;
    }
    println!("{}", required_fuel);
}
