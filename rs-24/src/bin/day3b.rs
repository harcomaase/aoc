fn main() {
    let input = std::fs::read_to_string("./inputs/3.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> i32 {
    let mut total = 0;
    let mut active = true;
    for line in input.lines() {
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();
        let l = chars.len();
        loop {
            if i >= l {
                break;
            }
            match chars[i] {
                'm' => {
                    if active
                        && l > i + 3
                        && chars[i + 1] == 'u'
                        && chars[i + 2] == 'l'
                        && chars[i + 3] == '('
                    {
                        println!("mul!");
                        match parse_number(&chars, i + 3, ',') {
                            Some((possible_a, len)) => {
                                println!("possible_a: {possible_a}");
                                match parse_number(&chars, i + 3 + len, ')') {
                                    Some((possible_b, len2)) => {
                                        println!("possible_b: {possible_b}");
                                        total += possible_a * possible_b;
                                        i += 3 + len + len2;
                                        continue;
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    }
                }
                'd' => {
                    if l > i + 3
                        && chars[i + 1] == 'o'
                        && chars[i + 2] == '('
                        && chars[i + 3] == ')'
                    {
                        active = true;
                        i += 3;
                        continue;
                    } else if l > i + 6
                        && chars[i + 1] == 'o'
                        && chars[i + 2] == 'n'
                        && chars[i + 3] == '\''
                        && chars[i + 4] == 't'
                        && chars[i + 5] == '('
                        && chars[i + 6] == ')'
                    {
                        active = false;
                        i += 6;
                        continue;
                    }
                }
                _ => (),
            }
            i += 1;
        }
    }
    total
}

fn parse_number(chars: &Vec<char>, offset: usize, terminal: char) -> Option<(i32, usize)> {
    let mut number = 0;
    let mut size = 0;
    loop {
        size += 1;
        if offset + size >= chars.len() {
            return None;
        }
        let c = chars[offset + size];
        if c == terminal {
            println!("terminal {terminal}");
            if size > 1 && size < 5 {
                //length of number plus terminal
                println!("parsed {number}");
                return Some((number, size));
            } else {
                return None;
            }
        }
        match c {
            '0'..='9' => {
                let parsed_digit = chars[offset + size].to_digit(10).unwrap() as i32;
                println!("parsed {parsed_digit}");
                if number == 0 {
                    number = parsed_digit;
                    continue;
                }
                number *= 10;
                number += parsed_digit;
            }
            _ => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input2() {
        let input = std::fs::read_to_string("./inputs/3-t2.txt").unwrap();

        assert_eq!(48, solve(input));
    }

    #[test]
    fn example_input2_1() {
        let input = std::fs::read_to_string("./inputs/3-t1.txt").unwrap();

        assert_eq!(161, solve(input));
    }
}
