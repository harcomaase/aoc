use std::fs;

// brute-force recursive approach, just to watch the world (and one of my processors) burn
//
// initially meant to have a comparison between the 0,1s better approach and this one, but
//   this brute-force approach is running since two hours, I need to sleep now :)
fn main() {
    let filename = "../input/20/day10.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut adapters: Vec<i32> = file
        .split("\n")
        .map(|s| s.parse::<i32>().expect("parsable number"))
        .collect();

    adapters.push(0); //outlet
    adapters.sort();

    println!("hmm: {}", rec(&adapters, 0));
}

fn rec(adapters: &Vec<i32>, index: usize) -> usize {
    let len = adapters.len();

    let mut count = 1;
    for i in index..len {
        if i + 2 < len && adapters[i + 2] - adapters[i] == 2 {
            count += rec(adapters, i + 2);
        }
        if i + 3 < len && adapters[i + 3] - adapters[i] == 3 {
            count += rec(adapters, i + 3);
        }
    }
    count
}
