use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./inputs/1.txt").unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a.push(split.next().unwrap());
        b.push(split.next().unwrap());
    }
    let map = create_map(&b);
    let mut score = 0;
    for i in a {
        score += i * *map.get(&i).unwrap_or(&0) as i32;
    }

    println!("{score}");
}

fn create_map(numbers: &Vec<i32>) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    for n in numbers {
        if map.contains_key(n) {
            let i = map.get_mut(n).unwrap();
            *i += 1;
        } else {
            map.insert(*n, 1);
        }
    }
    map
}
