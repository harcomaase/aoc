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
                let adjacent_occupied_seats =
                    get_adjacent_occupied_seats(x, y, &floor_plan, max_x, max_y);

                if *tile == 'L' && adjacent_occupied_seats == 0 {
                    copy[y][x] = '#';
                } else if *tile == '#' && adjacent_occupied_seats >= 5 {
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

fn get_adjacent_occupied_seats(
    x: usize,
    y: usize,
    floor_plan: &Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
) -> u8 {
    let mut adjacent_occupied_seats = 0;

    for yy in -1..=1 {
        for xx in -1..=1 {
            if xx == 0 && yy == 0 {
                continue;
            }
            adjacent_occupied_seats += match is_adjacent_occupied_seat_in_direction(
                x,
                y,
                floor_plan,
                (xx, yy),
                max_x,
                max_y,
            ) {
                true => 1,
                false => 0,
            };
        }
    }

    adjacent_occupied_seats
}

fn is_adjacent_occupied_seat_in_direction(
    x: usize,
    y: usize,
    floor_plan: &Vec<Vec<char>>,
    (dir_x, dir_y): (i64, i64),
    max_x: usize,
    max_y: usize,
) -> bool {
    let (mut xx, mut yy) = (x as i64, y as i64);
    loop {
        xx += dir_x;
        yy += dir_y;
        if xx < 0 || xx >= max_x as i64 || yy < 0 || yy >= max_y as i64 {
            return false;
        }
        match get_tile(xx as usize, yy as usize, floor_plan) {
            Some('#') => return true,
            Some('L') => return false,
            _ => (),
        }
    }
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
