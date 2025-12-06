use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day3.txt").unwrap();

    let mut priorities_sum = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let set1 = &chars[..chars.len() / 2];
        let set2 = &chars[chars.len() / 2..];

        let item = intersect_kind_of(set1, set2);
        let priority = {
            if item.is_ascii_uppercase() {
                (item as u8 - 38) as i32
            } else {
                (item as u8 - 96) as i32
            }
        };

        priorities_sum += priority;
    }

    println!("{}", priorities_sum);
}

fn intersect_kind_of(set1: &[char], set2: &[char]) -> char {
    for c in set1 {
        if set2.contains(c) {
            return *c;
        }
    }
    panic!("no intersection!")
}
