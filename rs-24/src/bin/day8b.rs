use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./inputs/8.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;
    let mut node_map: HashMap<char, Vec<(i32, i32)>> = HashMap::with_capacity(62);
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            let c = row[x];
            if c == '.' {
                continue;
            }
            let coords = (x as i32, y as i32);
            node_map
                .entry(c)
                .and_modify(|l| l.push(coords))
                .or_insert(vec![coords]);
        }
    }

    for nodes in node_map.values() {
        for (n1, (x1, y1)) in nodes.iter().enumerate() {
            for (n2, (x2, y2)) in nodes.iter().enumerate() {
                if n1 == n2 {
                    continue;
                }
                grid[*y1 as usize][*x1 as usize] = '#';

                let (dx, dy) = (x2 - x1, y2 - y1);
                let mut ax = *x2;
                let mut ay = *y2;
                loop {
                    ax += dx;
                    ay += dy;
                    if ax >= 0 && ax < max_x && ay >= 0 && ay < max_y {
                        grid[ay as usize][ax as usize] = '#';
                    } else {
                        break;
                    }
                }
                ax = *x1;
                ay = *y1;
                loop {
                    ax -= dx;
                    ay -= dy;
                    if ax >= 0 && ax < max_x && ay >= 0 && ay < max_y {
                        grid[ay as usize][ax as usize] = '#';
                    } else {
                        break;
                    }
                }
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/8-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(34, output);
    }
}
