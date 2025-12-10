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

    let mut total_button_presses = 0;
    for machine in &machines {
        // create permutations of button wirings (each button probably only
        // has to be pressed once, since two button presses change nothing)
        let indices: Vec<usize> = (0..machine.button_wirings.len()).collect();
        // try first 1 button press, then 2, then... n
        for permutation_length in 1.. {
            let button_press_sequence_permutations =
                permutations_with_repetitions(&indices, permutation_length);
            // println!("permutations: {button_press_sequence_permutations:?}");
            let mut success = false;
            for button_press_sequence in button_press_sequence_permutations {
                let mut joltages = vec![0; machine.indicator_diagram.len()];

                for button_press in button_press_sequence {
                    let button_wiring = &machine.button_wirings[button_press];
                    // increment joltage for each wired slot
                    for indicator_index in button_wiring {
                        let i = *indicator_index;
                        joltages[i] += 1;
                        if joltages[i] > machine.joltage_requirements[i] {
                            break;
                        }
                    }
                }
                if joltages == machine.joltage_requirements {
                    success = true;
                    break;
                }
            }
            if success {
                total_button_presses += permutation_length;
                break;
            }
        }
    }

    println!("{total_button_presses:?}");
}

fn permutations_with_repetitions(items: &Vec<usize>, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::with_capacity(items.len());
    perm_rec(items, k, &mut current, &mut result);
    result
}

fn perm_rec(items: &Vec<usize>, k: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }

    for i in 0..items.len() {
        current.push(items[i]);
        perm_rec(items, k, current, result);
        current.pop();
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
