fn main() {
    let input = std::fs::read_to_string("./inputs/5.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let rule_regex = regex::Regex::new(r"([\d]+)\|([\d]+)").unwrap();
    let mut split = input.split("\n\n");
    let rules: Vec<(i32, i32)> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let cap = rule_regex.captures(line).unwrap();
            (
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<i32>> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();
    let mut result = 0;
    for update in updates {
        let mut valid = true;
        for rule in &rules {
            if let Some(false) = validate(rule, &update) {
                valid = false;
                break;
            }
        }
        if valid {
            result += update[update.len() / 2];
        }
    }
    result
}

fn validate(rule: &(i32, i32), update: &Vec<i32>) -> Option<bool> {
    match index_of(&update, rule.0) {
        Some(i1) => {
            match index_of(&update, rule.1) {
                Some(i2) => {
                    // both values of rule are in update
                    return Some(i1 < i2);
                }
                None => None,
            }
        }
        None => None,
    }
}

fn index_of(vec: &Vec<i32>, n: i32) -> Option<usize> {
    for (i, v) in vec.iter().enumerate() {
        if *v == n {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input2() {
        let input = std::fs::read_to_string("./inputs/5-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(143, output);
    }
}
