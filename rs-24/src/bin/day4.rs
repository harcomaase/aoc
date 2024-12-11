fn main() {
    let input = std::fs::read_to_string("./inputs/4.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut instances = 0;
    let word: Vec<char> = "XMAS".chars().collect();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = grid[y][x];
            if c == word[0] {
                // found start letter
                // look right
                if check(&grid, &word, y, x, 0, 1) {
                    instances += 1;
                }
                // look left
                if check(&grid, &word, y, x, 0, -1) {
                    instances += 1;
                }
                // look up
                if check(&grid, &word, y, x, 1, 0) {
                    instances += 1;
                }
                // look down
                if check(&grid, &word, y, x, -1, 0) {
                    instances += 1;
                }
                // look up right
                if check(&grid, &word, y, x, 1, 1) {
                    instances += 1;
                }
                // look up left
                if check(&grid, &word, y, x, 1, -1) {
                    instances += 1;
                }
                // look down right
                if check(&grid, &word, y, x, -1, 1) {
                    instances += 1;
                }
                // look down left
                if check(&grid, &word, y, x, -1, -1) {
                    instances += 1;
                }
            }
        }
    }
    instances
}

fn check(grid: &Vec<Vec<char>>, word: &Vec<char>, y: usize, x: usize, dy: i32, dx: i32) -> bool {
    let mut x = x;
    let mut y = y;
    for i in 1..word.len() {
        let newy = y as i32 + dy;
        y = newy as usize;
        if let Some(r) = grid.get(y) {
            let newx = x as i32 + dx;
            x = newx as usize;
            if let Some(actual) = r.get(x) {
                if *actual != word[i] {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/4-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(4, output);
    }

    #[test]
    fn example_input2() {
        let input = std::fs::read_to_string("./inputs/4-t2.txt").unwrap();

        let output = solve(input);
        assert_eq!(18, output);
    }
}
