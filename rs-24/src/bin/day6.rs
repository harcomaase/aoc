use core::panic;

fn main() {
    let input = std::fs::read_to_string("./inputs/6.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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
        grid[y][x] = 'X';
        let (dx, dy) = directions[direction % directions.len()];
        let new_y = y as i32 + dy;
        let new_x = x as i32 + dx;
        // check if grid has been left
        if new_y >= max_y || new_y < 0 || new_x >= max_x || new_x < 0 {
            break;
        }
        // check next position in direction
        let next = get(&grid, new_y as usize, new_x as usize);
        if let Some('#') = next {
            // obstacle, change direction
            direction += 1;
        } else {
            // move there
            y = new_y as usize;
            x = new_x as usize;
        }
    }
    grid.iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count() as i32)
        .sum()
}

fn find(grid: &Vec<Vec<char>>, wanted: char) -> (usize, usize) {
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = grid[y][x];
            if c == wanted {
                return (y, x);
            }
        }
    }
    panic!()
}

fn get(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Option<char> {
    if let Some(row) = grid.get(y) {
        if let Some(c) = row.get(x) {
            Some(*c)
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
        assert_eq!(41, output);
    }
}
