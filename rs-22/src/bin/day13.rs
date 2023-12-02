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

    let packet_pairs = parse_input(&input);
    let mut right_order_packets = 0;

    for pair in packet_pairs {
        if right_order(pair) {
            right_order_packets += 1
        }
    }

    println!("{} packets are in the right order", right_order_packets);
}

fn right_order(pair: (Vec<Node>, Vec<Node>)) -> bool {
    let mut left = pair.0.iter();
    let mut right = pair.1.iter();

    loop {
        let l = left.next();
        let r = right.next();
        if l.is_none() && r.is_none() {
            println!("left:  {:?}", &pair.0);
            println!("right: {:?}", &pair.1);
            panic!(
                "reached end of both lists at once, and everything was equal? Should not happen!"
            )
        }
        if l.is_none() {
            return true;
        }
        if r.is_none() {
            // wrong order
            return false;
        }
        let l = l.unwrap();
        let r = r.unwrap();
        match &l.value {
            Value::Integer(left_int) => {
                match &r.value {
                    Value::Integer(right_int) => {
                        if left_int < right_int {
                            return true;
                        }
                        if right_int > left_int {
                            return false;
                        }
                    }
                    Value::List(right_list_indices) => {
                        // asdf
                    }
                }
            }
            Value::List(left_list_indices) => {
                match &r.value {
                    Value::Integer(right_int) => {
                        // compare
                    }
                    Value::List(right_list_indices) => {
                        // asdf
                    }
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vec<Node>, Vec<Node>)> {
    let pairs = input.split("\n\n");

    let mut result = Vec::new();
    for pair in pairs {
        let mut lines = pair.lines();
        let left = lines.next().unwrap();
        let right = lines.next().unwrap();
        assert_eq!(None, lines.next());

        let nodes1 = parse_packet(left);
        let nodes2 = parse_packet(right);

        result.push((nodes1, nodes2));
    }
    result
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
