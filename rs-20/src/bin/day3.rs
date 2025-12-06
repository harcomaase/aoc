use std::fs;

struct Vector {
    x: usize,
    y: usize,
}

fn main() {
    let filename = "../input/20/day3.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let map: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();
    let max_y = map.len();

    let dir = Vector { x: 3, y: 1 };

    let mut pos = Vector { x: 1, y: 1 };
    let mut collisions = 0;

    while pos.y < max_y {
        pos.x += dir.x;
        pos.y += dir.y;

        let line = map.get(pos.y - 1).expect("valid index");
        let normalized_x = (pos.x - 1) % line.len();
        let tile = line.get(normalized_x).expect("valid index");

        match tile {
            '#' => collisions += 1,
            _ => (),
        }
    }

    println!("collisions with trees: {}", collisions);
}
