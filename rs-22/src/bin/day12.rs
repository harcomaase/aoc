use std::fs;

#[derive(Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("../input/22/day12.txt").unwrap();

    let (map, start, end) = parse_input(&input);
    println!("start ({}|{}), end ({}|{})", start.x, start.y, end.x, end.y);

    let mut paths = go(&vec![start], &map, &end);
    paths.sort_by(|a, b| a.len().cmp(&b.len()));

    println!(
        "found {} paths, the shortest takes {} steps",
        paths.len(),
        paths[0].len() - 1 // minus one because the start tile is not counted
    );
}

fn go(steps: &Vec<Pos>, map: &Vec<Vec<u8>>, end: &Pos) -> Vec<Vec<Pos>> {
    let pos = steps.last().unwrap();

    if pos.eq(end) {
        return vec![steps.clone()];
    }

    let height = map[pos.y][pos.x];
    let mut paths = Vec::new();

    find_paths_in_direction(
        Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        height,
        steps,
        map,
        end,
        &mut paths,
    );
    if pos.y > 0 {
        find_paths_in_direction(
            Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            height,
            steps,
            map,
            end,
            &mut paths,
        );
    }
    find_paths_in_direction(
        Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        height,
        steps,
        map,
        end,
        &mut paths,
    );
    if pos.x > 0 {
        find_paths_in_direction(
            Pos {
                x: pos.x - 1,
                y: pos.y,
            },
            height,
            steps,
            map,
            end,
            &mut paths,
        );
    }

    paths
}

fn find_paths_in_direction(
    next_pos: Pos,
    height: u8,
    steps: &Vec<Pos>,
    map: &Vec<Vec<u8>>,
    end: &Pos,
    paths: &mut Vec<Vec<Pos>>,
) {
    check_direction(next_pos, height, steps, map, end)
        .into_iter()
        .filter(|p| !p.is_empty())
        .for_each(|p| paths.push(p));
}

fn check_direction(
    next_pos: Pos,
    current_height: u8,
    steps: &Vec<Pos>,
    map: &Vec<Vec<u8>>,
    end: &Pos,
) -> Vec<Vec<Pos>> {
    match map.get(next_pos.y).unwrap_or(&Vec::new()).get(next_pos.x) {
        Some(next_height) => {
            if *next_height <= current_height + 1 {
                // possible way
                // check if not already visited
                if !steps.contains(&next_pos) {
                    // go that way
                    let mut next_steps = steps.clone();
                    next_steps.push(next_pos);
                    return go(&next_steps, map, end);
                }
            }
        }
        None => (), // outside of map
    }
    Vec::new()
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Pos, Pos) {
    let mut result = Vec::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        let mut chars = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some(Pos { x, y });
                    chars.push('a' as u8 - 97);
                }
                'E' => {
                    end = Some(Pos { x, y });
                    chars.push('z' as u8 - 97);
                }
                _ => chars.push(c as u8 - 97),
            }
        }
        result.push(chars);
    }
    (result, start.unwrap(), end.unwrap())
}
