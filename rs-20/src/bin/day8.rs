use std::fs;

#[derive(Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
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

    let mut accumulator = 0;
    let mut pointer = 0;

    let mut visited_instruction_indices = vec![];

    loop {
        if visited_instruction_indices.contains(&pointer) {
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

    println!("accumulator value: {}", accumulator);
}
