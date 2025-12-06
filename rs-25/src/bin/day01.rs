use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").unwrap();

    let mut cur = 50;

    let mut count = 0;
    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value = value.parse::<i64>().unwrap();

        match dir {
            "L" => {
                cur -= value;
                while cur < 0 {
                    cur += 100;
                }
            }
            "R" => {
                cur += value;
                while cur > 99 {
                    cur -= 100;
                }
            }
            _ => panic!("unexpected character"),
        }

        if cur == 0 {
            count += 1;
        }
    }

    println!("{count}");
}
