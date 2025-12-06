use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").unwrap();

    let mut split = input.split("\n\n");
    let mut ranges: Vec<(usize, usize)> = split
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

    while let Some((new_range, i1, i2)) = check_join(&ranges) {
        ranges.remove(i1.max(i2));
        ranges.remove(i1.min(i2));
        ranges.push(new_range);
    }
    let fresh: usize = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
    println!("{fresh}");
}

fn check_join(ranges: &Vec<(usize, usize)>) -> Option<((usize, usize), usize, usize)> {
    for (i1, range1) in ranges.iter().enumerate() {
        for (i2, range2) in ranges.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            if range1.0 <= range2.1 && range2.0 <= range1.1 {
                // replace ranges by joint one
                let new_range = (range2.0.min(range1.0), range1.1.max(range2.1));
                return Some((new_range, i1, i2));
            }
        }
    }
    None
}
