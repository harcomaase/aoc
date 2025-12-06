use std::fs;

fn main() {
    let filename = "../input/20/day6.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let sum: usize = file
        .split("\n\n")
        .map(|s| s.replace(' ', ""))
        .map(|s| s.split('\n').map(|s| String::from(s)).collect())
        .map(|answers: Vec<String>| {
            let mut first = answers.get(0).expect("valid index").clone();
            for i in 1..answers.len() {
                let other = answers.get(i).expect("valid index");
                first.retain(|c| other.contains(c));
            }
            first.chars().collect()
        })
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
