use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day10.txt").unwrap();

    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    let mut checkpoint_signal_strengths = Vec::new();

    let mut cycle = 0;
    let mut x = 1;
    for instruction in input.lines() {
        let (cycles, value) = match &instruction[0..4] {
            "noop" => (1, 0),
            "addx" => {
                let value = instruction[5..].parse::<i32>().unwrap();
                (2, value)
            }
            _ => panic!("unexpected instruction: {}", instruction),
        };

        for _i in 0..cycles {
            cycle += 1;
            if checkpoints.contains(&cycle) {
                let signal_strength = cycle * x;
                println!(
                    "during {}th cycle, register x has the value {}, so the signal strength is {}",
                    cycle, x, signal_strength
                );
                checkpoint_signal_strengths.push(signal_strength);
            }
        }
        x += value;
    }

    println!("sum: {}", checkpoint_signal_strengths.iter().sum::<i32>());
}
