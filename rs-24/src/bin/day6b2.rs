use core::panic;

fn main() {
    let input = std::fs::read_to_string("./inputs/6.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut grid: Vec<Vec<Vec<char>>> = input
        .lines()
        .map(|line| line.chars().map(|c| vec![c]).collect())
        .collect();
    let (starty, startx) = find(&grid, '^');

    let mut nice_positions = 0;
    let guard_positions = find_guard_positions(&grid);
    for (gy, gx) in &guard_positions {
        // each guard position can be an obstacle, why not
        // ... but not the start position
        if *gy == starty && *gx == startx {
            continue;
        }
        let old_g = grid[*gy][*gx][0];
        grid[*gy][*gx][0] = '#';
        if is_looping(&mut grid, starty, startx) {
            nice_positions += 1;
        }
        grid[*gy][*gx][0] = old_g;
        // revert changes from looping check
        restore_grid(&mut grid);
        grid[starty][startx][0] = '^';
    }
    nice_positions
}

fn restore_grid(grid: &mut Vec<Vec<Vec<char>>>) {
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            grid[y][x].truncate(1);
        }
    }
}

fn find_guard_positions(grid: &Vec<Vec<Vec<char>>>) -> Vec<(usize, usize)> {
    // it's fine, we can copy once
    let mut grid = grid.to_vec();
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;
    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut direction = 0;
    // find start point
    let (mut y, mut x) = find(&grid, '^');
    loop {
        // mark current position with 'X'
        grid[y][x][0] = 'X';
        let (dx, dy) = directions[direction % directions.len()];
        let new_y = y as i32 + dy;
        let new_x = x as i32 + dx;
        // check if grid has been left
        if new_y >= max_y || new_y < 0 || new_x >= max_x || new_x < 0 {
            break;
        }
        // check next position in direction
        let next = get(&grid, new_y as usize, new_x as usize);
        let mut obstacle = false;
        if next.is_some() {
            let next = next.unwrap();
            if next.len() == 1 && next[0] == '#' {
                // obstacle, change direction
                direction += 1;
                obstacle = true;
            }
        }
        if !obstacle {
            // move there
            y = new_y as usize;
            x = new_x as usize;
        }
    }

    let mut result = Vec::new();
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = grid[y][x][0];
            if c == 'X' {
                result.push((y, x));
            }
        }
    }
    result
}

fn is_looping(grid: &mut Vec<Vec<Vec<char>>>, starty: usize, startx: usize) -> bool {
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;
    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut direction = 0;
    // find start point (obsolete, now passed as parameter)
    let (mut y, mut x) = (starty, startx);

    loop {
        // mark current position with direction
        let dir_index = direction % directions.len();
        let dir_index_char = char::from_digit(dir_index as u32, 10).unwrap();
        let current = &grid[y][x];
        if current.contains(&dir_index_char) {
            // guard is running circles
            return true;
        }
        grid[y][x].push(dir_index_char);
        let (dx, dy) = directions[dir_index];
        let new_y = y as i32 + dy;
        let new_x = x as i32 + dx;
        // check if grid has been left
        if new_y >= max_y || new_y < 0 || new_x >= max_x || new_x < 0 {
            return false;
        }
        // check next position in direction
        let next = get(&grid, new_y as usize, new_x as usize);
        // it's ugly, but works
        let mut obstacle = false;
        if next.is_some() {
            let next = next.unwrap();
            if next.len() == 1 && next[0] == '#' {
                // obstacle, change direction
                direction += 1;
                obstacle = true;
            }
        }
        if !obstacle {
            // move there
            y = new_y as usize;
            x = new_x as usize;
        }
    }
}

fn find(grid: &Vec<Vec<Vec<char>>>, wanted: char) -> (usize, usize) {
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = grid[y][x][0];
            if c == wanted {
                return (y, x);
            }
        }
    }
    panic!()
}

// wohoo, quick generics :D
fn get<T>(grid: &Vec<Vec<T>>, y: usize, x: usize) -> Option<&T> {
    if let Some(row) = grid.get(y) {
        if let Some(c) = row.get(x) {
            Some(c)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/6-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(6, output);
    }
}
