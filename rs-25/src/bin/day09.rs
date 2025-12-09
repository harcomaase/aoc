use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").unwrap();

    let red_tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .map(|s: Vec<i64>| (s[0], s[1]))
        .collect();

    let mut biggest_area = 0;
    for i1 in 0..red_tiles.len() {
        for i2 in (i1 + 1)..red_tiles.len() {
            let area = ((red_tiles[i1].0 - red_tiles[i2].0).abs() + 1)
                * ((red_tiles[i1].1 - red_tiles[i2].1).abs() + 1);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    println!("{biggest_area}");
}
