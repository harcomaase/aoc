use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/22/day4.txt").unwrap();

    let regex = Regex::new(r"([\d]+)-([\d]+),([\d]+)-([\d]+)").unwrap();

    let mut pairs = 0;
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

        if a >= x && b <= y || x >= a && y <= b {
            pairs += 1;
        }
    }
    println!("{}", pairs);
}
