use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/23/day2.txt").unwrap();

    let red = 12;
    let green = 13;
    let blue = 14;

    let parts_regex = Regex::new(r" ?([\d]+) ([rgb])").unwrap();

    let mut id_sum = 0;
    for (i, line) in input.lines().enumerate() {
        let id = i + 1;
        let mut id_valid = true;
        for revealed in line.split(':').nth(1).unwrap().trim_start().split(';') {
            for parts in revealed.split(',') {
                let caps = parts_regex.captures(parts).unwrap();
                let quantity = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                match caps.get(2).unwrap().as_str() {
                    "r" => {
                        if quantity > red {
                            id_valid = false;
                            break;
                        }
                    }
                    "g" => {
                        if quantity > green {
                            id_valid = false;
                            break;
                        }
                    }
                    "b" => {
                        if quantity > blue {
                            id_valid = false;
                            break;
                        }
                    }
                    _ => panic!("unexpected colour"),
                }
            }
            if !id_valid {
                break;
            }
        }

        if id_valid {
            id_sum += id;
        }
    }
    println!("sum of ids: {id_sum}");
}
