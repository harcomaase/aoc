use std::fs;

fn main() {
    let filename = "../input/20/day11.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut floor_plan: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();

    let mut copy = floor_plan.clone();

    let max_y = floor_plan.len();
    let max_x = floor_plan.get(0).expect("valid index").len();

    draw(&floor_plan);
    println!();

    loop {
        for y in 0..max_y {
            for x in 0..max_x {
                let tile = get_tile(x, y, &floor_plan).expect("valid tile");
                let adjacent_occupied_seats = get_adjacent_occupied_seats(x, y, &floor_plan);

                if *tile == 'L' && adjacent_occupied_seats == 0 {
                    copy[y][x] = '#';
                } else if *tile == '#' && adjacent_occupied_seats >= 4 {
                    copy[y][x] = 'L';
                } else {
                    copy[y][x] = *tile;
                }
            }
        }

        //update floor_map
        let mut changed = false;
        for y in 0..max_y {
            for x in 0..max_x {
                if floor_plan[y][x] != copy[y][x] {
                    changed = true;
                }
                floor_plan[y][x] = copy[y][x];
            }
        }
        if !changed {
            break;
        }
    }

    draw(&floor_plan);
    println!();

    let occupied_seats: usize = floor_plan
        .into_iter()
        .map(|row: Vec<char>| row.into_iter().filter(|c| *c == '#').count())
        .sum();

    println!("occupied_seats: {}", occupied_seats);
}

fn get_adjacent_occupied_seats(x: usize, y: usize, floor_plan: &Vec<Vec<char>>) -> u8 {
    let mut adjacent_occupied_seats = 0;
    for yy in if y == 0 { 0 } else { y - 1 }..y + 2 {
        for xx in if x == 0 { 0 } else { x - 1 }..x + 2 {
            if xx == x && yy == y {
                continue;
            }
            adjacent_occupied_seats += match get_tile(xx, yy, floor_plan) {
                Some('#') => 1,
                _ => 0,
            };
        }
    }
    adjacent_occupied_seats
}

fn get_tile(x: usize, y: usize, floor_plan: &Vec<Vec<char>>) -> Option<&char> {
    let row = &floor_plan.get(y)?;
    row.get(x)
}

fn draw(floor_plan: &Vec<Vec<char>>) {
    for y in 0..floor_plan.len() {
        let row = &floor_plan.get(y).expect("valid index");
        for x in 0..row.len() {
            let tile = row.get(x).expect("valid index");
            print!("{}", tile);
        }
        println!();
    }
}
