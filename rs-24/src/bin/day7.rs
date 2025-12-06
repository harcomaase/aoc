fn main() {
    let input = std::fs::read_to_string("./inputs/7.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let mut split = line.split(':');
        let test_value = split.next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if equals(test_value, numbers[0], &numbers[1..]) {
            result += test_value;
        }
    }
    result
}

fn equals(test_value: u64, n: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return test_value == n;
    }
    for op in &OPS {
        if equals(test_value, op.operate(n, numbers[0]), &numbers[1..]) {
            return true;
        }
    }
    false
}

const OPS: [Operator; 2] = [Operator::ADD, Operator::MULTIPLY];

enum Operator {
    ADD,
    MULTIPLY,
}

impl Operator {
    fn operate(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::ADD => a + b,
            Self::MULTIPLY => a * b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/7-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(3749, output);
    }
}
