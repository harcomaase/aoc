use std::fs;

#[derive(Debug)]
struct Node {
    value: Value,
    parent: usize,
}

#[derive(Debug)]
enum Value {
    Integer(i64),
    List(Vec<usize>),
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

        let nodes1 = parse_packet(left);
        let nodes2 = parse_packet(right);

        println!("{:?}", nodes1);
        println!("{:?}", nodes2);
        println!();
    }
}

fn parse_packet(packet: &str) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut current_list = 0;

    let chars: Vec<char> = packet.chars().collect();
    let mut buf = Vec::with_capacity(3);

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '[' => {
                let node = Node {
                    parent: current_list,
                    value: Value::List(Vec::new()),
                };
                let index = nodes.len();
                nodes.push(node);
                if index > 0 {
                    match &mut nodes[current_list].value {
                        Value::Integer(_) => panic!(),
                        Value::List(list) => list.push(index),
                    };
                    current_list = index;
                }
            }
            ']' => {
                if buf.len() > 0 {
                    let number = buf.iter().collect::<String>().parse().unwrap();
                    let node = Node {
                        parent: current_list,
                        value: Value::Integer(number),
                    };
                    let index = nodes.len();
                    nodes.push(node);
                    match &mut nodes[current_list].value {
                        Value::Integer(_) => panic!(),
                        Value::List(list) => list.push(index),
                    };
                    buf.clear();
                }
                current_list = nodes[current_list].parent;
                //TODO: make parent list current
            }
            '0'..='9' => {
                buf.push(c);
            }
            ',' => {
                if buf.len() > 0 {
                    let number = buf.iter().collect::<String>().parse().unwrap();
                    let node = Node {
                        parent: current_list,
                        value: Value::Integer(number),
                    };
                    let index = nodes.len();
                    nodes.push(node);
                    match &mut nodes[current_list].value {
                        Value::Integer(_) => panic!(),
                        Value::List(list) => list.push(index),
                    };
                    buf.clear();
                }
            }
            _ => panic!("unexpected character: {}", c),
        }
        i += 1;
    }

    nodes
}
