use std::fs;

fn main() {

    let filename = "../input/19/day1.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in file.lines() {
        println!("{}", line);
    }
}
