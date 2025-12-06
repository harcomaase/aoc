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
                for _ in 0..value {
                    cur -= 1;
                    if cur == 0 {
                        count += 1;
                    }
                    if cur < 0 {
                        cur = 99;
                    }
                }
            }
            "R" => {
                for _ in 0..value {
                    cur += 1;
                    if cur > 99 {
                        cur = 0;
                    }
                    if cur == 0 {
                        count += 1;
                    }
                }
            }
            _ => panic!("unexpected character"),
        }
    }

    println!("{count}");
}
