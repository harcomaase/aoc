use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day8.txt").unwrap();

    // parse input
    let mut trees: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - 48).collect();
        trees.push(row);
    }

    let mut highest_scenic_score = 0;
    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let scenic_score = calculate_scenic_score(*tree, &trees, x, y);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    println!("highest scenic score: {}", highest_scenic_score);
}

fn calculate_scenic_score(tree: u8, trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut visible_up = 0;
    for y_it in (0..y).rev() {
        visible_up += 1;
        if trees[y_it][x] >= tree {
            break;
        }
    }
    let mut visible_down = 0;
    for y_it in y + 1..trees.len() {
        visible_down += 1;
        if trees[y_it][x] >= tree {
            break;
        }
    }
    let mut visible_left = 0;
    let first_row = &trees[0];
    for x_it in (0..x).rev() {
        visible_left += 1;
        if trees[y][x_it] >= tree {
            break;
        }
    }
    let mut visible_right = 0;
    for x_it in x + 1..first_row.len() {
        visible_right += 1;
        if trees[y][x_it] >= tree {
            break;
        }
    }
    // println!("{} {} {} {} = {}", visible_up, visible_down, visible_left, visible_right, visible_up * visible_down * visible_left * visible_right);
    visible_up * visible_down * visible_left * visible_right
}
