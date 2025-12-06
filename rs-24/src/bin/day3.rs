fn main() {
    let input = std::fs::read_to_string("./inputs/3.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut total = 0;
    let regex = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for line in input.lines() {
        let results: i32 = regex
            .captures_iter(line)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                )
            })
            .map(|(x, y)| x * y)
            .sum();
        total += results;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/3-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(161, output);
    }
}
