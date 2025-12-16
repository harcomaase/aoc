use std::fs;

#[derive(Debug)]
struct Machine {
    indicator_diagram: Vec<bool>,
    button_wirings: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("inputs/day10-test.txt").unwrap();

    let machines = parse_machines(&input);

    //TODO: instead of brute-forcing permutations, we can start from the joltage_requirements,
    //      then ... idk, select most fitting buttons to reduce the highest||lowest first?

    for machine in machines {
        let mut wirings_ordered = machine.button_wirings.clone();
        wirings_ordered.sort_by(|a, b| b.len().cmp(&a.len()));

    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut m = Machine {
                indicator_diagram: Vec::new(),
                button_wirings: Vec::new(),
                joltage_requirements: Vec::new(),
            };
            let mut i = 0;
            let chars: Vec<char> = line.chars().collect();
            loop {
                let c = chars[i];
                match c {
                    '[' => {
                        let close_index = line[i..].find(']').unwrap();
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
                            .for_each(|n| m.joltage_requirements.push(n));
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
