use std::fs;
use std::io;
use std::{thread, time};

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
    let filename = "../input/19/day15.txt";
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

    let mut tiles: Vec<Tile> = Vec::new();

    tiles.push(Tile { x: 0, y: 0, id: 1 });

    let mut droid = Tile { x: 0, y: 0, id: 0 };

    let mut last_direction = 1;

    let mut steps = 0;

    while int_com.running {
        let direction = determine_move(&tiles, &droid, last_direction);
        let result = run_intcode_computer(&mut int_com, vec![direction], 1);
        let output = result.unwrap();
        if !int_com.running && output.is_empty() {
            break;
        }
        steps += 1;
        let status = output[0];
        let mut next_tile = Tile {
            id: status,
            ..droid
        };
        match direction {
            1 => next_tile.y += 1,
            2 => next_tile.y -= 1,
            3 => next_tile.x -= 1,
            4 => next_tile.x += 1,
            _ => panic!("unknown direction"),
        }
        tiles.push(next_tile);
        match status {
            0 => {}
            1 => {
                last_direction = direction;
                match direction {
                    1 => droid.y += 1,
                    2 => droid.y -= 1,
                    3 => droid.x -= 1,
                    4 => droid.x += 1,
                    _ => panic!("unknown direction"),
                };
            }
            2 => {
                match direction {
                    1 => droid.y += 1,
                    2 => droid.y -= 1,
                    3 => droid.x -= 1,
                    4 => droid.x += 1,
                    _ => panic!("unknown direction"),
                };
                println!("found the oxygen system!");
                // break;
            }
            _ => panic!("unknown status"),
        }

        if steps > 10 && droid.x == 0 && droid.y == 0 {
            break;
        }
    }
    print_tiles(&tiles, &droid);

    tiles[0].id = 3;
    let mut water_steps = 0;

    'water: loop {
        water_steps += 1;
        let water_tile_indices: Vec<usize> = find_tile_indices_by_id(&tiles, 3);

        let mut adjacent_tile_indices: Vec<usize> = Vec::new();
        for water_tile_index in water_tile_indices {
            let water_tile = &tiles[water_tile_index];

            adjacent_tile_indices
                .push(find_tile_index(&tiles, water_tile.x, water_tile.y + 1).unwrap());
            adjacent_tile_indices
                .push(find_tile_index(&tiles, water_tile.x, water_tile.y - 1).unwrap());
            adjacent_tile_indices
                .push(find_tile_index(&tiles, water_tile.x + 1, water_tile.y).unwrap());
            adjacent_tile_indices
                .push(find_tile_index(&tiles, water_tile.x - 1, water_tile.y).unwrap());
        }

        for adjacent_tile_index in adjacent_tile_indices {
            let mut adjacent_tile = &mut tiles[adjacent_tile_index];
            match adjacent_tile.id {
                1 => adjacent_tile.id = 3,
                2 => break 'water,
                _ => (),
            }
        }

        // print_tiles(&tiles, &droid);
        // let duration = time::Duration::from_millis(500);
        // thread::sleep(duration);
    }
    print_tiles(&tiles, &droid);

    println!("took {} steps", water_steps);
}

fn determine_move(tiles: &Vec<Tile>, droid: &Tile, last_direction: i64) -> i64 {
    let right_of = right_of(tiles, droid, last_direction);

    if right_of > 0 {
        return right_of;
    }

    //found nothing, ask the user:
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let input = buf.trim();

    match input {
        "w" | "1" => 1,
        "s" | "2" => 2,
        "a" | "3" => 3,
        "d" | "4" => 4,
        _ => panic!("unknown input"),
    }
}

fn right_of(tiles: &Vec<Tile>, droid: &Tile, last_direction: i64) -> i64 {
    let direction_right_of_droid = match last_direction {
        1 => 4,
        2 => 3,
        3 => 1,
        4 => 2,
        _ => panic!("bla"),
    };
    let tile_index_right_of_droid_opt = match direction_right_of_droid {
        1 => find_tile_index(tiles, droid.x, droid.y + 1),
        2 => find_tile_index(tiles, droid.x, droid.y - 1),
        3 => find_tile_index(tiles, droid.x - 1, droid.y),
        4 => find_tile_index(tiles, droid.x + 1, droid.y),
        _ => panic!("bla"),
    };
    if tile_index_right_of_droid_opt.is_none() {
        return direction_right_of_droid;
    }
    let tile_index_right_of_droid = tile_index_right_of_droid_opt.unwrap() as usize;
    let tile_opt = tiles.get(tile_index_right_of_droid);
    if tile_opt.is_none() || tile_opt.unwrap().id != 0 {
        return direction_right_of_droid;
    }

    let dir = match last_direction {
        1 => 3,
        2 => 4,
        3 => 2,
        4 => 1,
        _ => panic!("bla"),
    };
    right_of(tiles, droid, dir)
}

fn print_tiles(tiles: &Vec<Tile>, droid: &Tile) {
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

    for y in (min_y - 1..max_y + 1).rev() {
        for x in min_x - 1..max_x + 1 {
            if droid.x == x && droid.y == y {
                print!("D");
                continue;
            }
            match find_tile_index(tiles, x, y) {
                Some(index) => match tiles[index].id {
                    0 => print!("#"),
                    1 => print!(" "),
                    2 => print!("O"),
                    3 => print!("."),
                    _ => panic!("unknown color"),
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

fn find_tile_indices_by_id(tiles: &Vec<Tile>, id: i64) -> Vec<usize> {
    let mut result = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        if tile.id == id {
            result.push(index);
        }
    }
    result
}

fn run_intcode_computer(
    int_com: &mut IntcodeComputer,
    mut inputs: Vec<i64>,
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
                // println!("input value: {}", input);
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
