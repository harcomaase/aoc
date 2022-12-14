use std::fs;

#[derive(PartialEq)]
struct Pos {
    x: i16,
    y: i16,
}

fn main() {
    let input = fs::read_to_string("../input/22/day14.txt").unwrap();

    let sand_start = Pos { x: 500, y: 0 };

    let rocks = parse_input(input);
    let mut sands = Vec::new();

    'outer: loop {
        let mut sand = Pos { ..sand_start };
        let mut successive_movements_down = 0;

        'inner: loop {
            if successive_movements_down > 100 {
                break 'outer;
            }
            let down = Pos {
                x: sand.x,
                y: sand.y + 1,
            };
            if !rocks.contains(&down) && !sands.contains(&down) {
                sand.y = down.y;
                successive_movements_down += 1;
                continue;
            }
            let down_left = Pos {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            if !rocks.contains(&down_left) && !sands.contains(&down_left) {
                sand.x = down_left.x;
                sand.y = down_left.y;
                successive_movements_down = 0;
                continue;
            }
            let down_right = Pos {
                x: sand.x + 1,
                y: sand.y + 1,
            };
            if !rocks.contains(&down_right) && !sands.contains(&down_right) {
                sand.x = down_right.x;
                sand.y = down_right.y;
                successive_movements_down = 0;
                continue;
            }
            // sand will rest
            sands.push(sand);
            break 'inner;
        }
    }

    println!(
        "{} units of came to rest, everything else falls endlessly",
        sands.len()
    );
}

fn parse_input(input: String) -> Vec<Pos> {
    let mut rocks = Vec::new();
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
                    if !rocks.contains(&rock) {
                        rocks.push(rock);
                    }
                }
            } else {
                let from = std::cmp::min(c1[0], c2[0]);
                let to = std::cmp::max(c1[0], c2[0]);
                for x in from..=to {
                    let rock = Pos { x, y: c1[1] };
                    if !rocks.contains(&rock) {
                        rocks.push(rock);
                    }
                }
            }
        }
    }
    rocks
}
