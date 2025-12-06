use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/23/day2.txt").unwrap();

    let parts_regex = Regex::new(r" ?([\d]+) ([rgb])").unwrap();

    let mut power_sum = 0;
    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for revealed in line.split(':').nth(1).unwrap().trim_start().split(';') {
            for parts in revealed.split(',') {
                let caps = parts_regex.captures(parts).unwrap();
                let quantity = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                match caps.get(2).unwrap().as_str() {
                    "r" => {
                        if quantity > red {
                            red = quantity;
                        }
                    }
                    "g" => {
                        if quantity > green {
                            green = quantity;
                        }
                    }
                    "b" => {
                        if quantity > blue {
                            blue = quantity;
                        }
                    }
                    _ => panic!("unexpected colour"),
                }
            }
        }

        power_sum += red * green * blue;
    }
    println!("sum of powers: {power_sum}");
}
