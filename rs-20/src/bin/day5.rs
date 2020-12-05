use std::fs;

fn main() {
    let filename = "../input/20/day5.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let max_seat_id = file
        .split("\n")
        .map(|s| s.replace('F', "0"))
        .map(|s| s.replace('B', "1"))
        .map(|s| s.replace('R', "1"))
        .map(|s| s.replace('L', "0"))
        .map(|s| (String::from(&s[0..7]), String::from(&s[7..10])))
        .map(|(c1, c2)| (i32::from_str_radix(&c1, 2), i32::from_str_radix(&c2, 2)))
        .map(|(r1, r2)| (r1.expect("valid binary"), r2.expect("valid binary")))
        .map(|(row, column)| row * 8 + column)
        .max()
        .unwrap();

    println!("{}", max_seat_id);
}
