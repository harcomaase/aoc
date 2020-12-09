use std::fs;

fn main() {
    let filename = "../input/20/day9.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let numbers: Vec<usize> = file
        .split("\n")
        .map(|s| s.parse::<usize>().expect("parsable number"))
        .collect();

    let preamble = 25;

    println!(
        "finding invalid number for provided input with preamble={}",
        preamble
    );
    let invalid_number = find_invalid_number(&numbers, preamble).expect("valid invalid number :)");
    println!("found invalid number: {}", invalid_number);

    println!("trying to find encryption weakness");
    let encryption_weakness =
        find_encryption_weakness(&numbers, invalid_number).expect("encryption weakness");
    println!("encryption_weakness: {}", encryption_weakness);
}

fn find_encryption_weakness(numbers: &Vec<usize>, invalid_number: usize) -> Option<usize> {
    for i in 0..numbers.len() {
        let mut sum = 0;
        let mut smallest = usize::MAX;
        let mut largest = 0;
        for j in i..numbers.len() {
            let number = numbers.get(j).expect("valid index");
            if *number < smallest {
                smallest = *number;
            }
            if *number > largest {
                largest = *number;
            }
            sum += number;
            if sum == invalid_number {
                println!(
                    "found encryption weakness! Used {} numbers, smallest: {}, largest: {}",
                    j - i,
                    smallest,
                    largest
                );
                return Some(smallest + largest);
            }
            if sum > invalid_number {
                break;
            }
        }
    }

    None
}

fn find_invalid_number(numbers: &Vec<usize>, preamble: usize) -> Option<usize> {
    for i in preamble..numbers.len() {
        let current_number = numbers.get(i).expect("valid index");
        let previous_numbers = &numbers[i - preamble..i];

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
            return Some(*current_number);
        }
    }
    None
}
