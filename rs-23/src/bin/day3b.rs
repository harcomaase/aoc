use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../input/23/day3.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut map_of_potential_gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for y in 0..map.len() {
        let row = &map[y];
        let mut part_number = 0;
        for x in 0..row.len() {
            let c = &row[x];
            match *c {
                '0'..='9' => {
                    part_number *= 10;
                    part_number += c.to_digit(10).unwrap() as u64;
                }
                _ => {
                    if part_number != 0 {
                        // check if numbers are adjacent to signs
                        check_surrounding_signs(
                            part_number,
                            x,
                            y,
                            &map,
                            &mut map_of_potential_gears,
                        );
                        part_number = 0;
                    }
                }
            }
        }
        if part_number != 0 {
            // check if numbers are adjacent to signs
            check_surrounding_signs(
                part_number,
                row.len() - 1,
                y,
                &map,
                &mut map_of_potential_gears,
            );
        }
    }

    let sum:u64=map_of_potential_gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0]*v[1])
        .sum();
        
    println!("{sum}");
}

fn check_surrounding_signs(
    part_number: u64,
    x: usize,
    y: usize,
    map: &Vec<Vec<char>>,
    map_of_potential_gears: &mut HashMap<(usize, usize), Vec<u64>>,
) {
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
                '*' => {
                    map_of_potential_gears
                        .entry((cy, cx))
                        .and_modify(|v| v.push(part_number))
                        .or_insert(vec![part_number]);
                }
                _ => (),
            }
        }
    }
}
