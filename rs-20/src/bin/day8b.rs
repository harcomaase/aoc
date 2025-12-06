use std::fs;

#[derive(Debug, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Operation,
    arg: i64,
}

fn main() {
    let filename = "../input/20/day8.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let instructions: Vec<Instruction> = file
        .split("\n")
        .map(|s| s.split(" ").collect())
        .map(|input: Vec<&str>| Instruction {
            op: match input.get(0).expect("valid index") {
                &"acc" => Operation::Acc,
                &"jmp" => Operation::Jmp,
                &"nop" => Operation::Nop,
                default => panic!("unknown operation: {}", default),
            },
            arg: input
                .get(1)
                .expect("valid index")
                .parse()
                .expect("parsable number"),
        })
        .collect();

    for i in 0..instructions.len() {
        let mut copy = instructions.clone();
        let instruction = copy.get(i).expect("valid index");
        match instruction.op {
            Operation::Acc => continue,
            Operation::Jmp => {
                copy[i] = Instruction {
                    op: Operation::Nop,
                    arg: instruction.arg,
                }
            }

            Operation::Nop => {
                copy[i] = Instruction {
                    op: Operation::Jmp,
                    arg: instruction.arg,
                }
            }
        }
        let infite_loop_detected = execute_program(&copy);
        if !infite_loop_detected {
            println!("nice. Needed {} iterations", i);
            break;
        }
    }
}

fn execute_program(instructions: &Vec<Instruction>) -> bool {
    let mut accumulator = 0;
    let mut pointer = 0;

    let mut visited_instruction_indices = vec![];
    let mut infinite_loop = false;

    while pointer < instructions.len() {
        if visited_instruction_indices.contains(&pointer) {
            infinite_loop = true;
            break;
        }
        visited_instruction_indices.push(pointer);

        let instruction = instructions.get(pointer).expect("valid pointer index");
        // println!("pointer: {}, accumulator: {}", pointer, accumulator);
        // println!("processing {:?}", instruction);
        match instruction.op {
            Operation::Acc => {
                accumulator += instruction.arg;
                pointer += 1;
            }
            Operation::Jmp => {
                pointer = (pointer as i64 + instruction.arg) as usize;
            }
            Operation::Nop => {
                pointer += 1;
            }
        }
    }

    println!("infinite_loop = {}", infinite_loop);
    println!("accumulator value: {}", accumulator);

    infinite_loop
}
