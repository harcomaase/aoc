use std::fs;

fn main() {
    let filename = "../input/20/day10.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut adapters: Vec<i32> = file
        .split("\n")
        .map(|s| s.parse::<i32>().expect("parsable number"))
        .collect();

    adapters.push(0); //outlet
    adapters.sort();
    adapters.push(adapters.get(adapters.len() - 1).expect("valid index") + 3); //last adapter to device

    let mut arrangements: usize = 1;

    let mut i = 0;
    while i < adapters.len() {
        let adjacent_ones = find_adjacent_ones(&adapters, i);
        let (multiplier, step) = match adjacent_ones {
            0 | 1 => (1, 1), //next is 3 or 1 (followed by a 3), so no additional permutations
            2 => (2, 2),     //2 additional permutations
            3 => (4, 3),     //4 additional permutations
            4 => (7, 4), //only 7 additional permutations :) (because a jump by 4 steps is not possible)
            other => panic!("invalid number of adjacent_ones: {}", other), //there are no more than 4 adjacent ones
        };

        arrangements *= multiplier;
        i += step; //advance iterator by steps that were taken into account
    }

    println!("arrangements: {}", arrangements);
}

fn find_adjacent_ones(adapters: &Vec<i32>, index: usize) -> u32 {
    let mut adjacent_ones = 0;
    for i in index + 1..adapters.len() {
        let last = adapters.get(i - 1).expect("valid index");
        let this = adapters.get(i).expect("valid index");
        if this - last == 1 {
            adjacent_ones += 1;
        } else {
            break;
        }
    }
    adjacent_ones
}
