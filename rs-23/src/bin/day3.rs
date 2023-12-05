use std::fs;

fn main() {
    let input = fs::read_to_string("../input/23/day3.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;

    for y in 0..map.len() {
        let row = &map[y];
        let mut part_number = 0;
        for x in 0..row.len() {
            let c = &row[x];
            match *c {
                '0'..='9' => {
                    part_number *= 10;
                    part_number += c.to_digit(10).unwrap();
                }
                _ => {
                    if part_number != 0 {
                        // check if numbers are adjacent to signs
                        if has_surrounding_sign(part_number, x, y, &map) {
                            sum += part_number;
                        }
                        part_number = 0;
                    }
                }
            }
        }
        if part_number != 0 {
            // check if numbers are adjacent to signs
            if has_surrounding_sign(part_number, row.len() - 1, y, &map) {
                sum += part_number;
            }
        }
    }

    println!("sum: {sum}");
}

fn has_surrounding_sign(part_number: u32, x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    let part_number_length = part_number.to_string().len();
    let mut start_x = x - part_number_length;
    if start_x > 0 {
        start_x -= 1;
    }
    let start_y = if y > 0 { y - 1 } else { y };
    let end_y = if y < map.len() - 1 { y + 1 } else { y };
    for cx in start_x..=x {
        for cy in start_y..=end_y {
            match map[cy][cx] {
                '0'..='9' | '.' => (),
                _ => return true,
            }
        }
    }
    false
}
