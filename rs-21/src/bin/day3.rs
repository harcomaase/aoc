fn main() {
    let file_content = std::fs::read_to_string("../input/21/day3.txt").expect("read input file");

    let input: Vec<&str> = file_content.lines().collect();

    let report_lines = input.len();
    let line_length = input.get(0).expect("reading input").len();
    let mut bit_counts: Vec<usize> = Vec::with_capacity(line_length);
    for _ in 0..line_length {
        bit_counts.push(0);
    }

    for line in input {
        line.chars().enumerate().for_each(|(i, c)| match c {
            '0' => (),
            '1' => bit_counts[i] += 1,
            _ => panic!("illegal input value: {}", c),
        })
    }

    let mut raw_gamma = String::new();
    let mut raw_epsilon = String::new();
    for bit_count in bit_counts {
        if bit_count > report_lines / 2 {
            raw_gamma.push('1');
            raw_epsilon.push('0');
        } else {
            raw_gamma.push('0');
            raw_epsilon.push('1');
        }
    }

    let gamma = i64::from_str_radix(&raw_gamma, 2).expect("parsing binary number");
    let epsilon = i64::from_str_radix(&raw_epsilon, 2).expect("parsing binary number");
    println!("gamma: {} -> {}", raw_gamma, gamma);
    println!("epsilon: {} -> {}", raw_epsilon, epsilon);
    println!("result: {}", gamma * epsilon);
}
