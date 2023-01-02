use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day25.txt").unwrap();

    let snafu_sum: i64 = input.lines().map(|snafu| snafu_to_decimal(snafu)).sum();

    println!("{}", snafu_sum);
    println!("{}", decimal_to_snafu(snafu_sum));
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let multi = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("unrecognised snafu digit: {}", c),
            };
            5_i64.pow(i as u32) * multi
        })
        .sum()
}

fn decimal_to_snafu(number: i64) -> String {
    let max = (0..20)
        .find(|i| {
            let max = 5_i64.pow(*i) * 2;
            return number < max;
        })
        .unwrap();
    let mut snafu = vec!['2'; (max + 1) as usize];

    for i in 0..=max {
        let _base = ['=', '-', '0', '1', '2']
            .iter()
            .find(|e| {
                snafu[i as usize] = **e;
                let check = snafu_to_decimal(&String::from_iter(&snafu));
                check >= number
            })
            .unwrap();
    }
    String::from_iter(snafu)
}
