use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").unwrap();

    let diagram: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut result = 0;
    for y in 0..diagram.len() {
        for x in 0..diagram[0].len() {
            // only process paper rolls
            if diagram[y][x] != '@' {
                continue;
            }
            // do not use negative coordinates
            let starty = if y == 0 { 0 } else { y - 1 };
            let startx = if x == 0 { 0 } else { x - 1 };

            let mut adjacent_rolls = 0;
            for yy in starty..=(y + 1) {
                for xx in startx..=(x + 1) {
                    // do not check the current x|y coords
                    if yy == y && xx == x {
                        continue;
                    }
                    // out of bounds check
                    if xx >= diagram[0].len() || yy >= diagram.len() {
                        continue;
                    }
                    if diagram[yy][xx] == '@' {
                        adjacent_rolls += 1;
                    }
                }
            }
            if adjacent_rolls < 4 {
                result += 1;
            }
        }
    }

    println!("{result}");
}
