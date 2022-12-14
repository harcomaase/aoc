use std::{fs, collections::HashSet};

#[derive(PartialEq, Hash, Eq)]
struct Pos {
    x: i16,
    y: i16,
}

fn main() {
    let input = fs::read_to_string("../input/22/day14.txt").unwrap();

    let sand_start = Pos { x: 500, y: 0 };

    let rocks = parse_input(input);

    let mut sands = HashSet::new();

    'outer: loop {
        let mut sand = Pos { ..sand_start };

        'inner: loop {
            let down = Pos {
                x: sand.x,
                y: sand.y + 1,
            };
            if !rocks.contains(&down) && !sands.contains(&down) {
                sand.y = down.y;
                continue;
            }
            let down_left = Pos {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            if !rocks.contains(&down_left) && !sands.contains(&down_left) {
                sand.x = down_left.x;
                sand.y = down_left.y;
                continue;
            }
            let down_right = Pos {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if !rocks.contains(&down_right) && !sands.contains(&down_right) {
                sand.x = down_right.x;
                sand.y = down_right.y;
                continue;
            }
            // sand will rest
            if sand.eq(&sand_start) {
                sands.insert(sand);
                break 'outer;
            }
            sands.insert(sand);
            break 'inner;
        }
    }

    println!(
        "{} units of came to rest, everything else falls endlessly",
        sands.len()
    );
}

fn parse_input(input: String) -> HashSet<Pos> {
    let mut rocks = HashSet::new();
    for line in input.lines() {
        let coords: Vec<&str> = line.split(" -> ").collect();
        for i in 1..coords.len() {
            let c1: Vec<i16> = coords[i - 1]
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect();
            let c2: Vec<i16> = coords[i].split(',').map(|v| v.parse().unwrap()).collect();
            if c1[0] == c2[0] {
                let from = std::cmp::min(c1[1], c2[1]);
                let to = std::cmp::max(c1[1], c2[1]);
                for y in from..=to {
                    let rock = Pos { x: c1[0], y };
                        rocks.insert(rock);
                }
            } else {
                let from = std::cmp::min(c1[0], c2[0]);
                let to = std::cmp::max(c1[0], c2[0]);
                for x in from..=to {
                    let rock = Pos { x, y: c1[1] };
                        rocks.insert(rock);
                }
            }
        }
    }

    let new_line_y = rocks.iter().map(|r| r.y).max().unwrap() + 2;
    let new_line_x1 = rocks.iter().map(|r| r.x).min().unwrap() - new_line_y * 2;
    let new_line_x2 = rocks.iter().map(|r| r.x).max().unwrap() + new_line_y * 2;

    for x in new_line_x1..=new_line_x2 {
        rocks.insert(Pos { x, y: new_line_y });
    }

    rocks
}
