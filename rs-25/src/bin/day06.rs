use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();

    let tasks: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let width = tasks[0].len();
    let mut grand_total: usize = 0;

    for i in 0..width {
        let sign = tasks[tasks.len() - 1][i];

        let mut total: usize = tasks[0][i].parse().unwrap();
        for task in &tasks[1..tasks.len() - 1] {
            let v: usize = task[i].parse().unwrap();
            match sign {
                "+" => total += v,
                "*" => total *= v,
                _ => panic!(),
            }
        }

        grand_total += total;
    }

    println!("{grand_total}");
}
