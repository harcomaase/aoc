use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "../input/20/day14.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let regex = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").expect("valid regex");

    let mut mask_zeros: u64 = u64::pow(2, 36) - 1; //AND
    let mut mask_ones: u64 = 0; //OR
    let mut parameters = HashMap::new();
    for line in file.lines() {
        if line.starts_with("mask = ") {
            //update the masks
            mask_zeros = u64::pow(2, 36) - 1; //AND
            mask_ones = 0; //OR
            for (i, b) in line.chars().rev().take(36).enumerate() {
                match b {
                    '0' => mask_zeros -= u64::pow(2, i as u32),
                    '1' => mask_ones += u64::pow(2, i as u32),
                    _ => (),
                }
            }
            continue;
        }

        let captures = regex.captures(line).expect("valid match");
        let index = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

        let masked_value = (value | mask_ones) & mask_zeros;

        parameters.insert(index, masked_value);
        println!("putting {} -> {}", index, masked_value);
    }

    let sum: u64 = parameters.values().sum();

    println!("sum: {} (size: {})", sum, parameters.values().len());
}
