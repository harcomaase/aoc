fn main() {
    let input = std::fs::read_to_string("./inputs/4.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut instances = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for y in 1..grid.len() {
        let row = &grid[y];
        for x in 1..row.len() {
            let c = grid[y][x];
            if c == 'A' {
                // found center letter

                let up_left = get(&grid, y + 1, x - 1);
                let up_right = get(&grid, y + 1, x + 1);
                let down_left = get(&grid, y - 1, x - 1);
                let down_right = get(&grid, y - 1, x + 1);
                if ((eq('M', up_right) && eq('S', down_left))
                    || (eq('S', up_right) && eq('M', down_left)))
                    && ((eq('M', down_right) && eq('S', up_left))
                        || (eq('S', down_right) && eq('M', up_left)))
                {
                    instances += 1;
                }
            }
        }
    }
    instances
}

fn eq(act: char, c: Option<char>) -> bool {
    match c {
        Some(test) => act == test,
        None => false,
    }
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
    fn example_input2() {
        let input = std::fs::read_to_string("./inputs/4-t2.txt").unwrap();

        let output = solve(input);
        assert_eq!(9, output);
    }
}
