use std::fs;

struct Instruction {
    opcode: u32,
    modes: [u32; 3],
}

struct IntcodeComputer {
    intcode: Vec<i64>,
    instruction_pointer: usize,
    relative_base: usize,
    running: bool,
}

struct Tile {
    x: i64,
    y: i64,
    id: i64,
}

fn main() {
    let filename = "../input/19/day17.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let intcode: Vec<i64> = file
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut int_com = IntcodeComputer {
        intcode: intcode,
        instruction_pointer: 0,
        relative_base: 0,
        running: true,
    };

    //let way = "L,10,R,8,L,6,R,6,L,8,L,8,R,8,L,10,R,8,L,6,R,6,R,8,L,6,L,10,L,10,L,10,R,8,L,6,R,6,L,8,L,8,R,8,R,8,L,6,L,10,L,10,L,8,L,8,R,8,R,8,L,6,L,10,L,10,L,8,L,8,R,8";
    let way = "A,B,A,C,A,B,C,B,C,B\n";
    let way_a = "L,10,R,8,L,6,R,6\n";
    let way_b = "L,8,L,8,R,8\n";
    let way_c = "R,8,L,6,L,10,L,10\n";

    let mut inputs: Vec<i64> = Vec::new();

    for c in way.chars() {
        inputs.push(c as i64);
    }
    for c in way_a.chars() {
        inputs.push(c as i64);
    }
    for c in way_b.chars() {
        inputs.push(c as i64);
    }
    for c in way_c.chars() {
        inputs.push(c as i64);
    }
    for c in "n\n".chars() {
        inputs.push(c as i64);
    }

    inputs.reverse();

    let mut tiles: Vec<Tile> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    int_com.intcode[0] = 2;

    while int_com.running {
        let result = run_intcode_computer(&mut int_com, &mut inputs, 1);
        let output = result.unwrap();
        if !int_com.running && output.is_empty() {
            break;
        }
        let status = output[0];
        println!("got status {}", status);
        match status {
            10 => {
                x = 0;
                y += 1;
                continue;
            }
            11..=120 => {
                //scaffold
                tiles.push(Tile { x, y, id: status });
            }
            _ => panic!("unknown tile"),
        };
        x += 1;
    }
    print_tiles(&tiles);
}

fn get_min_max(tiles: &Vec<Tile>) -> (i64, i64, i64, i64) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for tile in tiles {
        if tile.x < min_x {
            min_x = tile.x;
        }
        if tile.x > max_x {
            max_x = tile.x;
        }
        if tile.y < min_y {
            min_y = tile.y;
        }
        if tile.y > max_y {
            max_y = tile.y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

fn print_tiles(tiles: &Vec<Tile>) {
    let (min_x, max_x, min_y, max_y) = get_min_max(tiles);

    for y in min_y - 1..max_y + 1 {
        for x in min_x - 1..max_x + 1 {
            match find_tile_index(tiles, x, y) {
                Some(index) => match tiles[index].id {
                    _ => print!("{}", std::char::from_u32(tiles[index].id as u32).unwrap()),
                    // 35 => print!("#"),
                    // 46 => print!("."),
                    // 94 => print!("X"),
                    // _ => panic!("unknown color"),
                },
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn find_tile_index(tiles: &Vec<Tile>, x: i64, y: i64) -> Option<usize> {
    for (index, tile) in tiles.iter().enumerate() {
        if tile.x == x && tile.y == y {
            return Option::from(index);
        }
    }
    None
}

fn run_intcode_computer(
    int_com: &mut IntcodeComputer,
    inputs: &mut Vec<i64>,
    expected_output_length: usize,
) -> Option<Vec<i64>> {
    let mut outputs: Vec<i64> = Vec::new();
    loop {
        let input = *int_com.intcode.get(int_com.instruction_pointer).unwrap();
        let instruction = parse_instruction(input);

        match instruction.opcode {
            1 | 2 => {
                let term1 = get_input_parameter(int_com, 1, instruction.modes);
                let term2 = get_input_parameter(int_com, 2, instruction.modes);
                let i3 = get_output_parameter(int_com, 3, instruction.modes);
                let result = if instruction.opcode == 1 {
                    term1 + term2
                } else {
                    term1 * term2
                };
                ensure_capacity(&mut int_com.intcode, i3 as usize);
                // println!("setting position {} to value {}", i3, result);
                int_com.intcode[i3 as usize] = result;

                int_com.instruction_pointer += 4;
            }
            3 => {
                let input = inputs.pop().unwrap();
                println!("input value: {}", input);
                let pos: i64 = get_output_parameter(int_com, 1, instruction.modes);
                ensure_capacity(&mut int_com.intcode, pos as usize);
                // println!("writing value {} to position {}", input, pos);
                int_com.intcode[pos as usize] = input;

                int_com.instruction_pointer += 2;
            }
            4 => {
                let param = get_input_parameter(int_com, 1, instruction.modes);
                // println!("output: {}", param);
                outputs.push(param);
                if *int_com
                    .intcode
                    .get(int_com.instruction_pointer + 2)
                    .unwrap()
                    == 99
                {
                    println!("test run successful");
                    int_com.running = false;
                    return Option::from(outputs);
                }
                // if param != 0 {
                //     panic!("non-zero output code");
                // }
                int_com.instruction_pointer += 2;

                if outputs.len() == expected_output_length {
                    return Option::from(outputs);
                }
            }
            5 | 6 => {
                let param1 = get_input_parameter(int_com, 1, instruction.modes);

                if instruction.opcode == 5 && param1 == 0 || instruction.opcode == 6 && param1 != 0
                {
                    int_com.instruction_pointer += 3;
                    continue;
                }
                let param2 = get_input_parameter(int_com, 2, instruction.modes);

                // println!("jumping to {}", param2);
                int_com.instruction_pointer = param2 as usize;
            }
            7 | 8 => {
                let param1 = get_input_parameter(int_com, 1, instruction.modes);
                let param2 = get_input_parameter(int_com, 2, instruction.modes);
                let new_value = if instruction.opcode == 7 && param1 < param2
                    || instruction.opcode == 8 && param1 == param2
                {
                    1
                } else {
                    0
                };
                let i3 = get_output_parameter(int_com, 3, instruction.modes);
                // let i3 = *intcode.get(i + 3).unwrap();
                ensure_capacity(&mut int_com.intcode, i3 as usize);
                int_com.intcode[i3 as usize] = new_value;

                int_com.instruction_pointer += 4;
            }
            9 => {
                let param = get_input_parameter(int_com, 1, instruction.modes);

                // println!("changing relative base by {}", param);

                int_com.relative_base = (int_com.relative_base as i64 + param) as usize;

                int_com.instruction_pointer += 2;
            }
            99 => {
                println!("exiting unexpectedly, but well");
                int_com.running = false;
                return Option::from(outputs);
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

fn get_output_parameter(int_com: &mut IntcodeComputer, offset: usize, modes: [u32; 3]) -> i64 {
    let mode = modes[offset - 1];
    let value = *int_com
        .intcode
        .get(int_com.instruction_pointer + offset)
        .unwrap();
    match mode {
        0 => value,
        2 => int_com.relative_base as i64 + value,
        _ => panic!("invalid instruction mode for output parameters"),
    }
}

fn get_input_parameter(int_com: &mut IntcodeComputer, offset: usize, modes: [u32; 3]) -> i64 {
    let mode = modes[offset - 1];
    let value = *int_com
        .intcode
        .get(int_com.instruction_pointer + offset)
        .unwrap();
    match mode {
        0 => {
            ensure_capacity(&mut int_com.intcode, value as usize);
            *int_com.intcode.get(value as usize).unwrap()
        }
        1 => value,
        2 => {
            let pos = (int_com.relative_base as i64 + value) as usize;
            ensure_capacity(&mut int_com.intcode, pos);
            *int_com.intcode.get(pos).unwrap()
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
