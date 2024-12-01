fn main() {
    let input = std::fs::read_to_string("./inputs/1.txt").unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a.push(split.next().unwrap());
        b.push(split.next().unwrap());
    }
    a.sort();
    b.sort();
    let mut total = 0;
    for i in 0..a.len() {
        total += i32::abs(a[i] - b[i]);
    }

    println!("{total}");
}
