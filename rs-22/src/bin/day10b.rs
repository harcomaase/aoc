use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day10.txt").unwrap();

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

            let pixel_pos = (cycle - 1) % 40;

            if pixel_pos == 0 {
                println!();
            }
            
            if (x - 1..=x + 1).contains(&pixel_pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        x += value;
    }
    println!();
}
