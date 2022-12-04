use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day3.txt").unwrap();

    let mut priorities_sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let chars1: Vec<char> = lines[i].chars().collect();
        let chars2: Vec<char> = lines[i + 1].chars().collect();
        let chars3: Vec<char> = lines[i + 2].chars().collect();

        let intersect = intersections_kind_of(&intersections_kind_of(&chars1, &chars2), &chars3);

        assert_eq!(
            1,
            intersect.len(),
            "found more than 1 common letter: {:?}",
            intersect
        );

        let priority = {
            let item = intersect[0];
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

fn intersections_kind_of(set1: &[char], set2: &[char]) -> Vec<char> {
    let mut result = Vec::new();
    for c in set1 {
        if set2.contains(c) {
            result.push(*c);
        }
    }
    result.dedup();
    result
}
