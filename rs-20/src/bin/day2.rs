use regex::Regex;
use std::fs;

fn main() {
    let filename = "../input/20/day2.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let regex = Regex::new(r"^([0-9]+)\-([0-9]+) ([a-zA-Z]): ([a-zA-Z]+)").expect("valid regex");

    let mut valid_passwords = 0;

    for line in file.lines() {
        let captures = regex.captures(line).expect("successful capture");

        let min = captures
            .get(1)
            .expect("capture group 1")
            .as_str()
            .parse::<i32>()
            .expect("parsable number");
        let max = captures
            .get(2)
            .expect("capture group 2")
            .as_str()
            .parse::<i32>()
            .expect("parsable number");
        let character = captures
            .get(3)
            .expect("capture group 3")
            .as_str()
            .chars()
            .nth(0)
            .expect("at least one character");
        let password = captures.get(4).expect("capture group 4").as_str();

        let mut character_count = 0;
        for c in password.chars() {
            if c == character {
                character_count += 1;
            }
        }
        if character_count >= min && character_count <= max {
            valid_passwords += 1;
        }
    }

    println!("valid passwords: {}", valid_passwords);
}
