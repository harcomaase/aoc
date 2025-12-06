use std::fs;

fn main() {
    let filename = "../input/19/day1.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut required_fuel = 0;
    for line in file.lines() {
        if line.is_empty() {
            continue;
        }
        let mass = line.parse::<i32>().unwrap();
        required_fuel += calculate_fuel_complete(mass);
    }
    println!("{}", required_fuel);
}

fn calculate_fuel_complete(mass : i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + calculate_fuel_complete(fuel)
}
