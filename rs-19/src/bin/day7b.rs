use std::fs;

struct Instruction {
    opcode: u32,
    modes: [u32; 3],
}

struct Amplifier {
    intcode: Vec<i32>,
    i: usize,
    finished: bool,
}

fn main() {
    let filename = "../input/19/day7.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let intcode: Vec<i32> = file
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let mut highest_output = 0;

    let phase_setting_interval = 5..10;
    for amp_a in phase_setting_interval.clone() {
        for amp_b in phase_setting_interval.clone() {
            for amp_c in phase_setting_interval.clone() {
                for amp_d in phase_setting_interval.clone() {
                    for amp_e in phase_setting_interval.clone() {
                        let phase_settings = vec![amp_a, amp_b, amp_c, amp_d, amp_e];

                        if invalid_phase_settings(&phase_settings) {
                            continue;
                        }

                        let result = run_with_phase_settings(&intcode, &phase_settings);

                        if result > highest_output {
                            highest_output = result;
                        }

                        println!(
                            "phase_settings [{}, {}, {}, {}, {}] lead to output: {}",
                            phase_settings[0],
                            phase_settings[1],
                            phase_settings[2],
                            phase_settings[3],
                            phase_settings[4],
                            result
                        );
                    }
                }
            }
        }
    }

    println!("highest output: {}", highest_output);
}

fn invalid_phase_settings(phase_settings: &Vec<i32>) -> bool {
    let mut cloned = phase_settings.clone();
    cloned.sort();
    cloned.dedup();

    phase_settings.len() != cloned.len()
}

fn run_with_phase_settings(intcode: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut amplifiers = Vec::with_capacity(phase_settings.len());
    for _ in 0..phase_settings.len() {
        amplifiers.push(Amplifier {
            intcode: intcode.clone(),
            i: 0,
            finished: false,
        });
    }

    let mut input = 0;
    let mut first_run = true;
    while amplifiers_running(&amplifiers) {
        for (index, phase_setting) in phase_settings.iter().enumerate() {
            let mut inputs: Vec<i32> = Vec::new();
            inputs.push(input);
            if first_run {
                inputs.push(*phase_setting);
            }
            let output = run_amplifier(&mut amplifiers[index], &mut inputs);
            input = output.unwrap();
        }
        first_run = false;
    }
    input
}

fn amplifiers_running(amplifiers: &Vec<Amplifier>) -> bool {
    let mut finished = true;
    for a in amplifiers {
        finished &= a.finished;
    }
    !finished
}

fn run_amplifier(amplifier: &mut Amplifier, inputs: &mut Vec<i32>) -> Option<i32> {
    let size = amplifier.intcode.len();

    let mut i = amplifier.i;
    while i < size {
        // println!("now @ {}", i);
        let input = *amplifier.intcode.get(i).unwrap();
        let instruction = parse_instruction(input);

        // println!(
        //     "instruction (opcode={},modes={},{},{})",
        //     instruction.opcode, instruction.modes[0], instruction.modes[1], instruction.modes[2]
        // );

        match instruction.opcode {
            1 | 2 => {
                let (i1, i2, i3): (i32, i32, i32) = (
                    *amplifier.intcode.get(i + 1).unwrap(),
                    *amplifier.intcode.get(i + 2).unwrap(),
                    *amplifier.intcode.get(i + 3).unwrap(),
                );
                let term1 = if instruction.modes[0] == 0 {
                    *amplifier.intcode.get(i1 as usize).unwrap()
                } else {
                    i1
                };
                let term2 = if instruction.modes[1] == 0 {
                    *amplifier.intcode.get(i2 as usize).unwrap()
                } else {
                    i2
                };
                let result = if instruction.opcode == 1 {
                    term1 + term2
                } else {
                    term1 * term2
                };
                amplifier.intcode[i3 as usize] = result;

                i += 4;
            }
            3 => {
                let input = inputs.pop().unwrap();
                let pos: i32 = *amplifier.intcode.get(i + 1).unwrap();
                amplifier.intcode[pos as usize] = input;

                i += 2;
            }
            4 => {
                let mut param = *amplifier.intcode.get(i + 1).unwrap();
                if instruction.modes[0] == 0 {
                    param = *amplifier.intcode.get(param as usize).unwrap();
                }
                println!("output: {}", param);
                if *amplifier.intcode.get(i + 2).unwrap() == 99 {
                    // println!("test run successful");
                    amplifier.finished = true;
                    return Option::from(param);
                }
                i += 2;
                amplifier.i = i;
                return Option::from(param);
                // panic!("non-zero output code");
            }
            5 | 6 => {
                let (i1, i2): (i32, i32) = (
                    *amplifier.intcode.get(i + 1).unwrap(),
                    *amplifier.intcode.get(i + 2).unwrap(),
                );
                let param1 = if instruction.modes[0] == 0 {
                    *amplifier.intcode.get(i1 as usize).unwrap()
                } else {
                    i1
                };

                if instruction.opcode == 5 && param1 == 0 || instruction.opcode == 6 && param1 != 0
                {
                    i += 3;
                    continue;
                }
                let param2 = if instruction.modes[1] == 0 {
                    *amplifier.intcode.get(i2 as usize).unwrap()
                } else {
                    i2
                };

                // println!("jumping to {}", param2);
                i = param2 as usize;
            }
            7 | 8 => {
                let (i1, i2, i3): (i32, i32, i32) = (
                    *amplifier.intcode.get(i + 1).unwrap(),
                    *amplifier.intcode.get(i + 2).unwrap(),
                    *amplifier.intcode.get(i + 3).unwrap(),
                );
                let param1 = if instruction.modes[0] == 0 {
                    *amplifier.intcode.get(i1 as usize).unwrap()
                } else {
                    i1
                };
                let param2 = if instruction.modes[1] == 0 {
                    *amplifier.intcode.get(i2 as usize).unwrap()
                } else {
                    i2
                };
                let new_value = if instruction.opcode == 7 && param1 < param2
                    || instruction.opcode == 8 && param1 == param2
                {
                    1
                } else {
                    0
                };
                amplifier.intcode[i3 as usize] = new_value;

                i += 4;
            }
            _ => {
                println!("unknown opcode: {}", instruction.opcode);
                panic!("unknown opcode");
            }
        }
    }

    None
}

fn parse_instruction(input: i32) -> Instruction {
    let s: String = input.to_string();
    // println!("parsing instruction '{}'", s);
    let mut chars = s.chars().rev();
    let opcode = chars.next().unwrap().to_digit(10).unwrap()
        + 10 * chars.next().unwrap_or('0').to_digit(10).unwrap();

    Instruction {
        opcode: opcode,
        modes: [
            chars.next().unwrap_or('0').to_digit(10).unwrap(),
            chars.next().unwrap_or('0').to_digit(10).unwrap(),
            chars.next().unwrap_or('0').to_digit(10).unwrap(),
        ],
    }
}
