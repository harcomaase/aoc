use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/22/day5.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let stacks_quantity = (lines[0].chars().count() + 1) / 4;
    let empty_line_number = lines
        .iter()
        .enumerate()
        .find(|(_i, line)| line.trim().is_empty())
        .map(|(i, _line)| i)
        .unwrap();

    let mut stacks = parse_stacks(stacks_quantity, &lines, empty_line_number);

    let instruction_regex = Regex::new(r"move ([\d]+) from ([\d]+) to ([\d]+)").unwrap();

    for line in &lines[(empty_line_number + 1)..] {
        let captures = instruction_regex.captures(line).unwrap();
        let quantity = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let src = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let dest = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let mut buf = Vec::new();
        for _i in 0..quantity {
            let value = stacks[src].pop().unwrap();
            buf.push(value);
        }
        buf.reverse();
        stacks[dest].append(&mut buf);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}

fn parse_stacks(
    stacks_quantity: usize,
    lines: &Vec<&str>,
    empty_line_number: usize,
) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); stacks_quantity];
    for line in &lines[..(empty_line_number - 1)] {
        let chars: Vec<char> = line.chars().collect();
        for (i, pos) in (1..(stacks_quantity * 4)).step_by(4).enumerate() {
            let c = chars[pos];
            match c {
                ' ' => (),
                _ => stacks[i].push(c),
            }
        }
    }
    let mut result = Vec::with_capacity(stacks.len());
    for mut stack in stacks.into_iter() {
        stack.reverse();
        result.push(stack);
    }
    result
}
