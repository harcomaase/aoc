use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").unwrap();

    let mut split = input.split("\n\n");
    let ranges: Vec<(usize, usize)> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut range_split = line.split('-');
            (
                range_split.next().unwrap().parse().unwrap(),
                range_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let ingredient_ids: Vec<usize> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut fresh = 0;
    for ingredient_id in ingredient_ids {
        for range in &ranges {
            if ingredient_id >= range.0 && ingredient_id <= range.1 {
                fresh += 1;
                break;
            }
        }
    }
    println!("{fresh}");
}
