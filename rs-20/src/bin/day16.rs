use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Rule {
    range1: (usize, usize),
    range2: (usize, usize),
}

fn main() {
    let filename = "../input/20/day16.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let input: Vec<&str> = file.split("\n\n").collect();

    let raw_rules = input.get(0).expect("valid input part");
    // let my_ticket = input.get(1).expect("valid input part");
    let raw_other_tickets = input.get(2).expect("valid input part");

    let rules = parse_rules(raw_rules);
    println!("{:?}", rules);

    let other_tickets: Vec<Vec<usize>> = raw_other_tickets
        .lines()
        .skip(1)
        .map(|t| {
            t.split(",")
                .map(|n| n.parse().expect("parsable number"))
                .collect()
        })
        .collect();

    let mut ticket_scanning_error_rate = 0;
    for other_ticket in other_tickets {
        for value in other_ticket {
            let mut valid = false;
            for rule in &rules {
                if (value >= rule.range1.0 && value <= rule.range1.1)
                    || (value >= rule.range2.0 && value <= rule.range2.1)
                {
                    valid = true;
                    break;
                }
            }
            if !valid {
                println!("invalid value: {}", value);
                ticket_scanning_error_rate += value;
            }
        }
    }

    println!("ticket_scanning_error_rate: {}", ticket_scanning_error_rate);
}

fn parse_rules(raw_rules: &str) -> Vec<Rule> {
    let rules_regex =
        Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").expect("valid regex");
    let mut rules = vec![];
    for raw_rule in raw_rules.lines() {
        let captures = rules_regex.captures(raw_rule).expect("successful capture");

        let range1_low = captures
            .get(2)
            .expect("valid capture group 2")
            .as_str()
            .parse()
            .expect("parsable number");
        let range1_high = captures
            .get(3)
            .expect("valid capture group 3")
            .as_str()
            .parse()
            .expect("parsable number");
        let range2_low = captures
            .get(4)
            .expect("valid capture group 4")
            .as_str()
            .parse()
            .expect("parsable number");
        let range2_high = captures
            .get(5)
            .expect("valid capture group 5")
            .as_str()
            .parse()
            .expect("parsable number");

        rules.push(Rule {
            range1: (range1_low, range1_high),
            range2: (range2_low, range2_high),
        });
    }
    rules
}
