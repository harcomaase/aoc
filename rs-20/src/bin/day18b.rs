use regex::Regex;
use std::fs;

fn main() {
    let filename = "../input/20/day18.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let mut sum = 0;
    for line in file.lines() {
        let term = line.replace(" ", "");
        let subterm_results = evaluate_subterms(&term);
        let result = evaluate_term(&subterm_results);
        sum += result;

        println!("{} = {}", result, term);
    }

    println!("sum: {}", sum);
}

fn prepare_term(term: &str) -> String {
    let addition_regex = Regex::new(r"([0-9]+)\+([0-9]+)").expect("valid regex");

    let mut prepared_term = String::from(term);
    loop {
        let capturable_term = prepared_term.clone();
        let captures_opt = addition_regex.captures(&capturable_term);
        if captures_opt.is_none() {
            break;
        }
        let captures = captures_opt.expect("successful capture");
        if captures.len() == 1 {
            break;
        }
        let a = captures.get(1).expect("valid capture group");
        let b = captures.get(2).expect("valid capture group");

        let c = a.as_str().parse::<i64>().unwrap() + b.as_str().parse::<i64>().unwrap();

        prepared_term.replace_range(a.start()..b.end(), &c.to_string());
    }

    prepared_term
}

fn evaluate_subterms(term: &str) -> String {
    let subterm_regex = Regex::new(r"(\([0-9\+*]+\))").expect("valid regex");

    let mut prepared_term = String::from(term);
    loop {
        prepared_term = prepare_term(&prepared_term);
        let capturable_term = prepared_term.clone();
        let captures_opt = subterm_regex.captures(&capturable_term);
        if captures_opt.is_none() {
            break;
        }
        let captures = captures_opt.expect("successful capture");
        if captures.len() == 1 {
            break;
        }
        let subterm_capture = captures.get(1).expect("valid capture group");
        let subterm = subterm_capture.as_str();
        let result = evaluate_term(subterm);

        prepared_term.replace_range(subterm_capture.range(), &result.to_string());
    }

    prepared_term
}

fn evaluate_term(term: &str) -> i64 {
    let mut number = String::new();
    let mut result = -1;
    let term_chars: Vec<char> = term.chars().collect();
    let mut last_sign = '#';
    let mut open_parenthesis_counter = 0;
    let mut i = 0;
    while i <= term_chars.len() {
        let c = if i == term_chars.len() {
            '+'
        } else {
            *term_chars.get(i).expect("valid index")
        };
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                number.insert(number.len(), c)
            }
            '+' | '*' => {
                if number.len() == 0 {
                    //sign after parenthesis
                    last_sign = c;
                    i += 1;
                    continue;
                }
                let parsed = number.parse().expect("parsable number");
                if result == -1 {
                    result = parsed;
                    last_sign = c;
                } else {
                    match last_sign {
                        '+' => result += parsed,
                        '*' => result *= parsed,
                        _ => panic!("unexpected sign: {}", last_sign),
                    }
                    last_sign = c;
                }
                number.clear();
            }
            // ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
            '(' => {
                for j in i + 1..term_chars.len() {
                    match term_chars.get(j).expect("valid index") {
                        '(' => open_parenthesis_counter += 1,
                        ')' => {
                            if open_parenthesis_counter > 0 {
                                open_parenthesis_counter -= 1;
                                continue;
                            }
                            let subterm_result = evaluate_term(&term[i + 1..j]);
                            if result == -1 {
                                result = subterm_result;
                            } else {
                                match last_sign {
                                    '+' => result += subterm_result,
                                    '*' => result *= subterm_result,
                                    _ => panic!("unexpected sign: {}", last_sign),
                                }
                            }
                            i = j;
                            break;
                        }
                        _ => (),
                    }
                }
            }
            _ => panic!("unexpected character: {}", c),
        }
        i += 1;
    }
    result
}
