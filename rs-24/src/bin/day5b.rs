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
    let mut broken_updates = Vec::new();
    // check which updates are broken
    for update in updates {
        for rule in &rules {
            if let Some(false) = validate_rule(rule, &update) {
                broken_updates.push(update.clone());
                break;
            }
        }
    }
    // fix broken updates by swapping values when a rule matches but is currently invalid
    for mut broken_update in broken_updates {
        while !validate(&broken_update, &rules) {
            fix_update(&mut broken_update, &rules);
        }
        result += broken_update[broken_update.len() / 2];
    }
    result
}

fn validate(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for rule in rules {
        if let Some(false) = validate_rule(rule, update) {
            return false;
        }
    }
    true
}

fn fix_update(broken_update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) {
    for rule in rules {
        if let Some(false) = validate_rule(rule, &broken_update) {
            let i1 = index_of(&broken_update, rule.0).unwrap();
            let i2 = index_of(&broken_update, rule.1).unwrap();
            broken_update.swap(i1, i2);
        }
    }
}

fn validate_rule(rule: &(i32, i32), update: &Vec<i32>) -> Option<bool> {
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
        assert_eq!(123, output);
    }
}
