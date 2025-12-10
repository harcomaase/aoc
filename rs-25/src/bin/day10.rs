use std::fs;

#[derive(Debug)]
struct Machine {
    indicator_diagram: Vec<bool>,
    button_wirings: Vec<Vec<usize>>,
    joltage_requirments: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("inputs/day10-test.txt").unwrap();

    let machines = parse_machines(&input);

    let total_button_presses = 0;
    for machine in &machines {
        let mut indicator_lights = vec![false; machine.indicator_diagram.len()];

        //TODO: create permutations of button wirings
        //TODO: check if some button combinations are better than others

    }

    println!("{total_button_presses:?}");
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut m = Machine {
                indicator_diagram: Vec::new(),
                button_wirings: Vec::new(),
                joltage_requirments: Vec::new(),
            };
            let mut i = 0;
            let chars: Vec<char> = line.chars().collect();
            loop {
                let c = chars[i];
                match c {
                    '[' => {
                        let close_index = line[i..].find(']').unwrap();
                        println!("close_index: {close_index}");
                        println!("substring: {}", &line[(i + 1)..(i + close_index)]);
                        for cc in line[(i + 1)..(i + close_index)].chars() {
                            match cc {
                                '#' => m.indicator_diagram.push(true),
                                '.' => m.indicator_diagram.push(false),
                                _ => panic!("unexpected char in indicator diagram: {cc}"),
                            }
                        }
                        i += close_index;
                    }
                    '(' => {
                        let close_index = line[i..].find(')').unwrap();
                        let button_wiring = line[(i + 1)..(i + close_index)]
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect();
                        m.button_wirings.push(button_wiring);
                        i += close_index;
                    }
                    '{' => {
                        let close_index = line[i..].find('}').unwrap();
                        line[(i + 1)..(i + close_index)]
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .for_each(|n| m.joltage_requirments.push(n));
                        i += close_index;
                    }
                    _ => panic!("unexpected char: {c}"),
                }
                i += 2;
                if i >= chars.len() {
                    break;
                }
            }
            m
        })
        .collect()
}
