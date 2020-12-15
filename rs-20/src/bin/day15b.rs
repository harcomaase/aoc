use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "../input/20/day15.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut mem = HashMap::new();
    let mut last_turn_number = 0;
    file.split(",")
        .map(|s| s.parse::<usize>().expect("parsable number"))
        .enumerate()
        .for_each(|(index, number)| {
            mem.insert(number, vec![index + 1]);
            println!("turn {}, announcing {}", index + 1, number);
            last_turn_number = number;
        });

    for turn in 1 + mem.len()..=30000000 {
        let number = match mem.get_mut(&last_turn_number) {
            Some(last_turns) => {
                if last_turns.len() == 1 {
                    mem.get_mut(&0).unwrap().push(turn);
                    0
                } else {
                    let len = last_turns.len();
                    let diff = last_turns[len - 1] - last_turns[len - 2];
                    match mem.get_mut(&diff) {
                        Some(vec) => vec.push(turn),
                        None => {
                            mem.insert(diff, vec![turn]);
                        }
                    }
                    diff
                }
            }
            None => {
                mem.get_mut(&0).unwrap().push(turn);
                0
            }
        };

        last_turn_number = number;
    }

    println!("last spoken number: {}", last_turn_number);
}
