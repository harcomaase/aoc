use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day8.txt").unwrap();

    // parse input
    let mut trees: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - 48).collect();
        trees.push(row);
    }

    let mut visible_trees: Vec<(usize, usize)> = Vec::new();
    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if is_visible(*tree, &trees, x, y) {
                visible_trees.push((x, y));
            }
        }
    }
    println!("visible trees quantity: {}", visible_trees.len());
}

fn is_visible(tree: u8, trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let mut visible = true;
    for y_it in 0..y {
        if trees[y_it][x] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    for y_it in y + 1..trees.len() {
        if trees[y_it][x] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    let first_row = &trees[0];
    for x_it in 0..x {
        if trees[y][x_it] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    for x_it in x + 1..first_row.len() {
        if trees[y][x_it] >= tree {
            visible = false;
            break;
        }
    }
    visible
}
