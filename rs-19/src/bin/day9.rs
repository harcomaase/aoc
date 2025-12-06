use std::fs;
use std::io;

struct Instruction {
    opcode: u32,
    modes: [u32; 3],
}

fn main() {
    let filename = "../input/19/day9.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut intcode: Vec<i64> = file
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut relative_base = 0;
    let mut i = 0;
    loop {
        let input = *intcode.get(i).unwrap();
        let instruction = parse_instruction(input);

        match instruction.opcode {
            1 | 2 => {
                let term1 =
                    get_input_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);
                let term2 =
                    get_input_parameter(&mut intcode, i + 2, instruction.modes[1], relative_base);
                let i3 =
                    get_output_parameter(&mut intcode, i + 3, instruction.modes[2], relative_base);
                // let i3 = *intcode.get(i + 3).unwrap();
                let result = if instruction.opcode == 1 {
                    term1 + term2
                } else {
                    term1 * term2
                };
                ensure_capacity(&mut intcode, i3 as usize);
                // println!("setting position {} to value {}", i3, result);
                intcode[i3 as usize] = result;

                i += 4;
            }
            3 => {
                println!("input value: ");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let input = buf.trim().parse::<i64>().unwrap();
                let pos: i64 =
                    get_output_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);
                // let pos: i64 = *intcode.get(i + 1).unwrap();
                ensure_capacity(&mut intcode, pos as usize);
                // println!("writing value {} to position {}", input, pos);
                intcode[pos as usize] = input;

                i += 2;
            }
            4 => {
                let param =
                    get_input_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);
                println!("output: {}", param);
                if *intcode.get(i + 2).unwrap() == 99 {
                    println!("test run successful");
                    break;
                }
                if param != 0 {
                    panic!("non-zero output code");
                }
                i += 2;
            }
            5 | 6 => {
                let param1 =
                    get_input_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);

                if instruction.opcode == 5 && param1 == 0 || instruction.opcode == 6 && param1 != 0
                {
                    i += 3;
                    continue;
                }
                let param2 =
                    get_input_parameter(&mut intcode, i + 2, instruction.modes[1], relative_base);

                // println!("jumping to {}", param2);
                i = param2 as usize;
            }
            7 | 8 => {
                let param1 =
                    get_input_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);
                let param2 =
                    get_input_parameter(&mut intcode, i + 2, instruction.modes[1], relative_base);
                let new_value = if instruction.opcode == 7 && param1 < param2
                    || instruction.opcode == 8 && param1 == param2
                {
                    1
                } else {
                    0
                };
                let i3 =
                    get_output_parameter(&mut intcode, i + 3, instruction.modes[2], relative_base);
                // let i3 = *intcode.get(i + 3).unwrap();
                ensure_capacity(&mut intcode, i3 as usize);
                intcode[i3 as usize] = new_value;

                i += 4;
            }
            9 => {
                let param =
                    get_input_parameter(&mut intcode, i + 1, instruction.modes[0], relative_base);

                // println!("changing relative base by {}", param);

                relative_base += param;

                i += 2;
            }
            _ => {
                println!("unknown opcode: {}", instruction.opcode);
                panic!("unknown opcode");
            }
        }
    }
}

fn ensure_capacity(intcode: &mut Vec<i64>, i: usize) {
    if i > intcode.len() - 1 {
        intcode.resize(i + 1, 0);
    }
}

fn get_output_parameter(intcode: &mut Vec<i64>, i: usize, mode: u32, relative_base: i64) -> i64 {
    let value = *intcode.get(i).unwrap();
    match mode {
        0 => value,
        2 => relative_base + value,
        _ => panic!("invalid instruction mode for output parameters"),
    }
}

fn get_input_parameter(intcode: &mut Vec<i64>, i: usize, mode: u32, relative_base: i64) -> i64 {
    let value = *intcode.get(i).unwrap();
     match mode {
        0 => {
            ensure_capacity(intcode, value as usize);
            *intcode.get(value as usize).unwrap()
        }
        1 => value,
        2 => {
            let pos = relative_base + value;
            ensure_capacity(intcode, pos as usize);
            *intcode.get(pos as usize).unwrap()
        }
        _ => {
            println!("unknown instruction mode: {}", mode);
            panic!("unknown instruction mode");
        }
    }
}

fn parse_instruction(input: i64) -> Instruction {
    let s: String = input.to_string();
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
