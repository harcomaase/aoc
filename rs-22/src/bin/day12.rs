use std::{collections::VecDeque, fs};

struct Pos {
    x: i32,
    y: i32,
    height: u8,
    visited: bool,
    steps: usize,
}

fn main() {
    let input = fs::read_to_string("../input/22/day12.txt").unwrap();

    let (mut map, start, mut end) = parse_input(&input);

    let mut queue = VecDeque::with_capacity(map.len() * map[0].len());
    queue.push_back((start.x, start.y));

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    'outer: loop {
        let current = queue.pop_front().unwrap();
        let current_steps = map[current.1 as usize][current.0 as usize].steps;
        let current_height = map[current.1 as usize][current.0 as usize].height;
        for direction in directions {
            let next_x = current.0 + direction.0;
            let next_y = current.1 + direction.1;
            if next_x < 0 || next_y < 0 {
                continue;
            }
            let next_x = next_x as usize;
            let next_y = next_y as usize;
            if next_y >= map.len() || next_x >= map[0].len() {
                continue;
            }
            let next = &mut map[next_y][next_x];
            if next.visited {
                continue;
            }
            if next.height > current_height + 1 {
                continue;
            }
            next.steps = current_steps + 1;
            next.visited = true;
            if next.x == end.x && next.y == end.y {
                end.steps = next.steps;
                break 'outer;
            }
            queue.push_back((next.x, next.y));
        }
    }

    println!("shortest path: {} steps", end.steps);
}

fn parse_input(input: &str) -> (Vec<Vec<Pos>>, Pos, Pos) {
    let mut result = Vec::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let height = match c {
                'S' => {
                    let h = 'a' as u8 - 97;
                    start = Some(Pos {
                        x,
                        y,
                        height: h,
                        visited: true,
                        steps: 0,
                    });
                    h
                }
                'E' => {
                    let h = 'z' as u8 - 97;
                    end = Some(Pos {
                        x,
                        y,
                        height: h,
                        visited: false,
                        steps: 0,
                    });
                    h
                }
                _ => c as u8 - 97,
            };
            row.push(Pos {
                x,
                y,
                height,
                visited: false,
                steps: 0,
            })
        }
        result.push(row);
    }
    (result, start.unwrap(), end.unwrap())
}
