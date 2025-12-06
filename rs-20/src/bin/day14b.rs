use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "../input/20/day14.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let regex = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").expect("valid regex");

    let mut mask = vec![];
    let mut mask_zeros: u64 = u64::pow(2, 36) - 1; //AND
    let mut mask_ones: u64 = 0; //OR
    let mut parameters = HashMap::new();
    for line in file.lines() {
        if line.starts_with("mask = ") {
            //update the masks
            mask = line.chars().rev().take(36).collect();
            mask_zeros = u64::pow(2, 36) - 1; //AND
            mask_ones = 0; //OR
            for (i, b) in line.chars().rev().take(36).enumerate() {
                match b {
                    'X' => mask_zeros -= u64::pow(2, i as u32),
                    '1' => mask_ones += u64::pow(2, i as u32),
                    _ => (),
                }
            }
            continue;
        }

        let captures = regex.captures(line).expect("valid match");
        let raw_index = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

        let updated_index = (raw_index | mask_ones) & mask_zeros;

        let indices = determine_indices(updated_index, &mask);

        for index in indices {
            parameters.insert(index, value);
        }
    }

    let sum: u64 = parameters.values().sum();

    println!("sum: {} (size: {})", sum, parameters.values().len());
}

fn determine_indices(index: u64, mask: &Vec<char>) -> Vec<u64> {
    let mut indices = vec![index];
    for (i, c) in mask.into_iter().enumerate() {
        if *c != 'X' {
            continue;
        }
        let mut tmp = vec![];
        for index in &indices {
            tmp.push(*index + u64::pow(2, i as u32));
        }
        indices.append(&mut tmp);
    }
    indices
}
