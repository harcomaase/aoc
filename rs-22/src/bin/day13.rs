use std::fs;

enum Value {
    Integer(i64),
    List(Vec<Value>),
}

fn main() {
    let input = fs::read_to_string("../input/22/day13_test1.txt").unwrap();

    parse_input(&input);
}

fn parse_input(input: &str) {
    let pairs = input.split("\n\n");

    for pair in pairs {
        let mut lines = pair.lines();
        let left = lines.next().unwrap();
        let right = lines.next().unwrap();
        assert_eq!(None, lines.next());

        parse_packet(left);
        parse_packet(right);
    }
}

fn parse_packet(packet: &str) {
    let mut current: Vec<Value> = Vec::new();

    let chars: Vec<char> = packet.chars().collect();

    let mut i = 1;
    while i < chars.len() - 1 {
        let c = chars[i];
        match c {
            '[' => {
                let nested_list = Vec::new();
                current.push(Value::List(nested_list))
                //TODO: make nested_list current, keep track of branches in this tree
            }
            ']' => (),
            '0'..='9' => {
                // collect numbers until ']' or ','
            }
            ',' => (),
            _ => panic!("unexpected character: {}", c),
        }
        i += 1;
    }
}
