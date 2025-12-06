use std::fs;

struct Vector {
    x: usize,
    y: usize,
}

fn main() {
    let filename = "../input/20/day3.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let map: Vec<Vec<char>> = file.split("\n").map(|s| s.chars().collect()).collect();
    let directions = vec![
        Vector { x: 1, y: 1 },
        Vector { x: 3, y: 1 },
        Vector { x: 5, y: 1 },
        Vector { x: 7, y: 1 },
        Vector { x: 1, y: 2 },
    ];

    let product: i64 = directions
        .into_iter()
        .map(|dir| determine_collisions(&map, dir))
        .product();
    println!("product of collisions with trees: {}", product);
}

fn determine_collisions(map: &Vec<Vec<char>>, dir: Vector) -> i64 {
    let max_y = map.len();
    let mut collisions = 0;
    let mut pos = Vector { x: 1, y: 1 };

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

    collisions
}
