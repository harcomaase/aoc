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

    let mut count_one_joltage_differences = 0;
    let mut count_three_joltages_differences = 1; //last adapter to device

    for i in 0..adapters.len() - 1 {
        let joltage1 = adapters.get(i).expect("valid index");
        let joltage2 = adapters.get(i + 1).expect("valid index");

        let difference = joltage2 - joltage1;
        match difference {
            1 => count_one_joltage_differences += 1,
            2 => (),
            3 => count_three_joltages_differences += 1,
            default => panic!("invalid joltage difference: {}", default),
        }
    }

    println!(
        "one jolt differences: {}, three jolt differences: {}, result: {}",
        count_one_joltage_differences,
        count_three_joltages_differences,
        count_one_joltage_differences * count_three_joltages_differences
    );
}
