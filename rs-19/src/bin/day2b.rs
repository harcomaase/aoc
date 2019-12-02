use std::fs;

fn main() {
    let filename = "../input/19/day2.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let intcode: Vec<i32> = file.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let expected_output: i32 = 19690720;
    let mut noun = 0;
    let mut verb = 0;

    'outer: for n in 0..99 {
        for v in 0..99 {
            if calculate_output(&intcode, n, v) == expected_output {
                noun = n;
                verb = v;
                break 'outer;
            }
        }
    }

    let result = 100 * noun + verb;

    println!("noun: {}, verb: {}", noun, verb);
    println!("result: {}", result);
}

fn calculate_output(intcode_original: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut intcode: Vec<i32> = intcode_original.clone();

    intcode[1] = noun;
    intcode[2] = verb;

    for i in (0..intcode.len()).step_by(4) {
        let opcode = *intcode.get(i).unwrap();
        if opcode == 99 {
            break;
        }
        let (i1, i2, i3): (i32, i32, i32) = (
            *intcode.get(i + 1).unwrap(),
            *intcode.get(i + 2).unwrap(),
            *intcode.get(i + 3).unwrap(),
        );
        let term1 = intcode.get(i1 as usize).unwrap();
        let term2 = intcode.get(i2 as usize).unwrap();
        let result = if opcode == 1 {
            term1 + term2
        } else {
            term1 * term2
        };

        intcode[i3 as usize] = result;
    }

    return intcode[0];
}
