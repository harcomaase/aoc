use std::fs;

fn main() {
    let filename = "../input/20/day6.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let sum: usize = file
        .split("\n\n")
        .map(|s| s.replace('\n', ""))
        .map(|s| s.replace(' ', ""))
        .map(|s| s.chars().collect())
        .map(|chars: Vec<char>| {
            let mut clone = chars.clone();
            clone.sort();
            clone.dedup();
            clone
        })
        .map(|chars| chars.len())
        .sum();

    println!("{}", sum);
}
