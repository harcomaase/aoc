use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();

    let count_of_batteries = 12;
    let mut joltage = 0;
    for line in input.lines() {
        // println!("checking {line}");
        let batteries: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|n| n as usize)
            .collect();

        // find first occurrences of each digit
        let mut digit_positions = [None; 10];
        for (i, battery) in batteries.iter().enumerate() {
            if digit_positions[*battery] == None {
                digit_positions[*battery] = Some(i);
            }
            // we could add a check if enough digits have been located already
        }

        // find the highest battery that is not after the last possible position
        for n in (1..=9).rev() {
            if let Some(index) = digit_positions[n] {
                if index > batteries.len() - count_of_batteries {
                    // after last possible position, does not work
                    continue;
                }
                // println!("  first digit: {n}");
                let mut highest_next_digit_index = index;
                let mut bank_joltage = n;
                for next_digit in 1..count_of_batteries {
                    // now start from this index and look for the highest digits again
                    let mut highest_next_digit = 0;
                    // println!(
                    //     "  checking: {:?}",
                    //     &batteries[(highest_next_digit_index + 1)..]
                    // );
                    for next_index in (highest_next_digit_index + 1)..batteries.len() {
                        if next_index > batteries.len() - (12 - next_digit) {
                            break;
                        }
                        let digit = batteries[next_index];
                        if digit == 9 {
                            highest_next_digit = digit;
                            highest_next_digit_index = next_index;
                            break;
                        }
                        if digit > highest_next_digit {
                            highest_next_digit = digit;
                            highest_next_digit_index = next_index;
                        }
                    }
                    // println!("  chosen: {highest_next_digit}");
                    bank_joltage = (bank_joltage * 10) + highest_next_digit;
                }
                // println!(" -> {bank_joltage}");
                joltage += bank_joltage;
                break;
            }
        }
    }
    println!("{joltage}");
}
