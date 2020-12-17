use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
struct Rule {
    field: String,
    range1: (usize, usize),
    range2: (usize, usize),
}

fn main() {
    let filename = "../input/20/day16.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let input: Vec<&str> = file.split("\n\n").collect();

    let raw_rules = input.get(0).expect("valid input part");
    let my_ticket: Vec<usize> = input
        .get(1)
        .expect("valid input part")
        .lines()
        .nth(1)
        .map(|t| {
            t.split(",")
                .map(|n| n.parse().expect("parsable number"))
                .collect()
        })
        .unwrap();
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

    let valid_tickets = find_valid_tickets(&other_tickets, &rules);

    let mut possibilities: Vec<Vec<Rule>> = Vec::new();
    for _i in 0..rules.len() {
        possibilities.push(rules.clone());
    }

    //remove invalid tickets
    for valid_ticket in valid_tickets {
        for (index, value) in valid_ticket.into_iter().enumerate() {
            let p = possibilities.get_mut(index).expect("valid index");
            let mut to_remove_indices = vec![];
            for (index, rule) in p.into_iter().enumerate().rev() {
                if (value >= rule.range1.0 && value <= rule.range1.1)
                    || (value >= rule.range2.0 && value <= rule.range2.1)
                {
                    //nice
                } else {
                    to_remove_indices.push(index);
                    break;
                }
            }
            for to_remove_index in to_remove_indices {
                p.remove(to_remove_index);
            }
        }
    }

    //deduct possibilities
    for i in (2..=possibilities.len()).rev() {
        let p_prev: Vec<Rule> = find_vec_with_len(&possibilities, i - 1).to_vec();
        let p: &mut Vec<Rule> = find_vec_with_len_mut(&mut possibilities, i);
        let mut j = 0;
        while j != p.len() {
            if p_prev
                .clone()
                .into_iter()
                .any(|other_rule| p[j].field.eq(&other_rule.field))
            {
                p.remove(j);
            } else {
                j += 1;
            }
        }
    }

    let mut result = 1;
    for (index, p) in possibilities.into_iter().enumerate() {
        println!("{:?}", p);
        if p[0].field.starts_with("departure") {
            result *= my_ticket.get(index).expect("valid index");
            println!(
                "{} multiplying {}, result: {}",
                p[0].field,
                my_ticket.get(index).expect("valid index"),
                result
            );
        }
    }
    println!("{}", result);
}
fn find_vec_with_len(possibilities: &Vec<Vec<Rule>>, len: usize) -> &Vec<Rule> {
    for p in possibilities {
        if p.len() == len {
            return p;
        }
    }
    panic!("mission impossible");
}

fn find_vec_with_len_mut(possibilities: &mut Vec<Vec<Rule>>, len: usize) -> &mut Vec<Rule> {
    for p in possibilities {
        if p.len() == len {
            return p;
        }
    }
    panic!("mission impossible");
}

fn find_valid_tickets(tickets: &Vec<Vec<usize>>, rules: &Vec<Rule>) -> Vec<Vec<usize>> {
    let mut valid_tickets = tickets.clone();
    for (index, other_ticket) in tickets.into_iter().enumerate().rev() {
        let mut valid_ticket = true;
        for value in other_ticket {
            let mut valid_value = false;
            for rule in rules {
                if (value >= &rule.range1.0 && value <= &rule.range1.1)
                    || (value >= &rule.range2.0 && value <= &rule.range2.1)
                {
                    valid_value = true;
                    break;
                }
            }
            if !valid_value {
                valid_ticket = false;
                break;
            }
        }
        if !valid_ticket {
            valid_tickets.remove(index);
        }
    }
    valid_tickets
}

fn parse_rules(raw_rules: &str) -> Vec<Rule> {
    let rules_regex =
        Regex::new(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").expect("valid regex");
    let mut rules = vec![];
    for raw_rule in raw_rules.lines() {
        let captures = rules_regex.captures(raw_rule).expect("successful capture");

        let field = captures.get(1).expect("valid capture group 1").as_str();
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
            field: String::from(field),
            range1: (range1_low, range1_high),
            range2: (range2_low, range2_high),
        });
    }
    rules
}
