use std::fs;

fn main() {
    let filename = "../input/20/day9.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let numbers: Vec<i64> = file
        .split("\n")
        .map(|s| s.parse::<i64>().expect("parsable number"))
        .collect();

    let preamble = 25;
    let considered_previous_numbers = 25;

    for i in preamble..numbers.len() {
        let current_number = numbers.get(i).expect("valid index");
        let previous_numbers = &numbers[i - considered_previous_numbers..i];

        let mut valid_number = false;
        'middle_loop: for j in 0..previous_numbers.len() {
            let previous_number1 = previous_numbers.get(j).expect("valid index");
            for k in j..previous_numbers.len() {
                let previous_number2 = previous_numbers.get(k).expect("valid index");
                if previous_number1 == previous_number2 {
                    continue;
                }
                if previous_number1 + previous_number2 == *current_number {
                    valid_number = true;
                    break 'middle_loop;
                }
            }
        }
        if !valid_number {
            println!("first invalid number: {}", current_number);
            break;
        }
    }
}
