use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();

    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();

    let mut tasks: Vec<Vec<String>> = Vec::new();
    let mut current_task = Vec::new();
    for i in 0..width {
        let sign_row_value = lines[lines.len() - 1][i];
        if sign_row_value != ' ' {
            if i > 0 {
                tasks.push(current_task);
                current_task = Vec::new();
            }
            current_task.push(sign_row_value.to_string());
        }
        let mut vertical_number = Vec::new();
        for li in 0..(lines.len() - 1) {
            let c = lines[li][i];
            if c != ' ' {
                vertical_number.push(c);
            }
        }
        let num: String = vertical_number.iter().collect();
        if !num.is_empty() {
            current_task.push(num);
        }
    }
    // do not forget the last column
    tasks.push(current_task);

    let mut grand_total: usize = 0;
    for task in tasks {
        let sign = task[0].as_str();
        let mut total: usize = task[1].parse().unwrap();
        for i in 2..task.len() {
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
