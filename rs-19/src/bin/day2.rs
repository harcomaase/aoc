use std::fs;

fn main() {
    let filename = "../input/19/day2.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut intcode : Vec<i32> = file.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let size = intcode.len();

    intcode[1] = 12;
    intcode[2] = 2;

    for i in (0..size).step_by(4) {
        let opcode = *intcode.get(i).unwrap();
        if opcode == 99 {
            break;
        }
        let (i1, i2, i3):(i32,i32,i32) = (*intcode.get(i + 1).unwrap(), *intcode.get(i + 2).unwrap(), *intcode.get(i + 3).unwrap());
        let term1 = intcode.get(i1 as usize).unwrap();
        let term2 = intcode.get(i2 as usize).unwrap();
        let result = if opcode == 1 {term1 + term2} else {term1 * term2};

        intcode[i3 as usize] = result;
    }
    
    println!("{:?}", intcode);
    println!("result: {}", intcode[0]);
}
