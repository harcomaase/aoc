use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();

    let mut joltage = 0;
    for line in input.lines() {
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

        // find the highest battery that is not at the last position
        for n in (1..=9).rev() {
            if let Some(index) = digit_positions[n] {
                if index == batteries.len() - 1 {
                    // last position, does not work
                    continue;
                }
                // now start from this index and look for the highest digits again
                let mut highest_second_digit = 0;
                for second_index in (index + 1)..batteries.len() {
                    let digit = batteries[second_index];
                    if digit == 9 {
                        highest_second_digit = digit;
                        break;
                    }
                    if digit > highest_second_digit {
                        highest_second_digit = digit;
                    }
                }
                let bank_joltage = (n * 10) + highest_second_digit;
                joltage += bank_joltage;
                break;
            }
        }
    }
    println!("{joltage}");
}
