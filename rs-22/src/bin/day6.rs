use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day6.txt").unwrap();

    let chars: Vec<char> = input.chars().collect();

    let start_of_packet_marker_length = 4;

    for i in 0..(chars.len() - start_of_packet_marker_length) {
        let end = i + start_of_packet_marker_length;
        if !contains_duplicates(&chars[i..end]) {
            println!("{}", end);
            break;
        }
    }
}

fn contains_duplicates(slice: &[char]) -> bool {
    let mut buf = Vec::new();
    for c in slice {
        if buf.contains(c) {
            return true;
        }
        buf.push(*c);
    }
    false
}
