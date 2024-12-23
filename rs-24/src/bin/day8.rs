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
                let (dx, dy) = (x2 - x1, y2 - y1);
                let antinode1 = (x2 + dx, y2 + dy);
                let antinode2 = (x1 - dx, y1 - dy);
                if antinode1.0 >= 0
                    && antinode1.0 < max_x
                    && antinode1.1 >= 0
                    && antinode1.1 < max_y
                {
                    grid[antinode1.1 as usize][antinode1.0 as usize] = '#';
                }
                if antinode2.0 >= 0
                    && antinode2.0 < max_x
                    && antinode2.1 >= 0
                    && antinode2.1 < max_y
                {
                    grid[antinode2.1 as usize][antinode2.0 as usize] = '#';
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
        assert_eq!(14, output);
    }
}
