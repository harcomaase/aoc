use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").unwrap();

    let mut result = 0;
    for range in input.split(',') {
        let mut range_split = range.split('-');
        let start: u64 = range_split.next().unwrap().trim().parse().unwrap();
        let end: u64 = range_split.next().unwrap().trim().parse().unwrap();

        for i in start..=end {
            let s = i.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let (part1, part2) = s.split_at(s.len() / 2);
            if part1.eq(part2) {
                result += i;
            }
        }
    }

    println!("{result}");
}
