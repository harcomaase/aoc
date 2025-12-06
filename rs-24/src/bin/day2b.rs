fn main() {
    let input = std::fs::read_to_string("./inputs/2.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut total = 0;
    for report in input.lines() {
        let levels: Vec<i32> = report
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut more_levels = vec![levels.clone()];
        for i in 0..levels.len() {
            let mut l = levels.clone();
            l.remove(i);
            more_levels.push(l);
        }

        for l in more_levels {
            if check(&l) {
                total += 1;
                break;
            }
        }
    }
    total
}

fn check(report: &Vec<i32>) -> bool {
    let mut levels = report.iter();

    let mut last = levels.next().unwrap();
    let mut ord = Ord::UNSURE;
    for level in levels {
        let diff = i32::abs(level - last);
        if diff > 3 || diff < 1 {
            return false;
        }
        match last - level {
            ..=-1 => {
                if ord == Ord::UNSURE {
                    ord = Ord::ASC;
                } else if ord == Ord::DESC {
                    return false;
                }
            }
            0 => (), // can not happen
            1.. => {
                if ord == Ord::UNSURE {
                    ord = Ord::DESC;
                } else if ord == Ord::ASC {
                    return false;
                }
            }
        }
        last = level;
    }
    true
}

#[derive(PartialEq)]
enum Ord {
    UNSURE,
    ASC,
    DESC,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/2-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(4, output);
    }
}
