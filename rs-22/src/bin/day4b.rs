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

        if intersections_kind_of(&(a..=b).collect(), &(x..=y).collect()).len() > 0 {
            pairs += 1;
        }
    }
    println!("{}", pairs);
}

fn intersections_kind_of(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in set1 {
        if set2.contains(i) {
            result.push(*i);
        }
    }
    result
}
