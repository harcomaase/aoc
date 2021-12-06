fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day3.txt").expect("read input file");

    let input: Vec<&str> = file_content.lines().collect();

    let raw_o2_rating = find_o2_rating(&input, 0);
    let raw_co2_rating = find_co2_rating(&input, 0);

    let o2_rating = i64::from_str_radix(&raw_o2_rating, 2).expect("parsing binary number");
    let co2_rating = i64::from_str_radix(&raw_co2_rating, 2).expect("parsing binary number");
    println!("o2_rating: {} -> {}", raw_o2_rating, o2_rating);
    println!("co2_rating: {} -> {}", raw_co2_rating, co2_rating);
    println!("result: {}", o2_rating * co2_rating);
}

fn find_o2_rating(lines: &Vec<&str>, pos: usize) -> String {
    let thingy = find_most_common_thingy(lines, pos);
    let filtered_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.chars().nth(pos).expect("") == thingy)
        .map(|line| *line)
        .collect();
    match filtered_lines.len() {
        0 => panic!("error in filtering"),
        1 => String::from(*filtered_lines.first().expect("filtering lines")),
        _ => find_o2_rating(&filtered_lines, pos + 1),
    }
}
fn find_co2_rating(lines: &Vec<&str>, pos: usize) -> String {
    let thingy = find_most_common_thingy(lines, pos);
    let filtered_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.chars().nth(pos).expect("") != thingy)
        .map(|line| *line)
        .collect();
    match filtered_lines.len() {
        0 => panic!("error in filtering"),
        1 => String::from(*filtered_lines.first().expect("filtering lines")),
        _ => find_co2_rating(&filtered_lines, pos + 1),
    }
}

fn find_most_common_thingy(lines: &Vec<&str>, pos: usize) -> char {
    let mut count = 0;
    for line in lines {
        if line.chars().nth(pos).expect("parsing input line") == '1' {
            count += 1;
        }
    }
    if count >= lines.len() - count {
        '1'
    } else {
        '0'
    }
}
