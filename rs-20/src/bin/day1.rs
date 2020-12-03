use std::fs;

fn main() {
    let filename = "../input/20/day1.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let entries: Vec<i32> = file
        .split("\n")
        .map(|s| s.parse::<i32>().expect("parsable number"))
        .collect();

    println!("{:?}", find_specific_entry_sum_and_multiply(2020, entries));
}

fn find_specific_entry_sum_and_multiply(sum: i32, entries: Vec<i32>) -> Option<i32> {
    let size = entries.len();
    for i in 0..size {
        let a = entries.get(i).expect("valid index");
        for j in i..size {
            let b = entries.get(j).expect("valid index");

            if a + b == sum {
                return Some(a * b);
            }
        }
    }
    None
}
