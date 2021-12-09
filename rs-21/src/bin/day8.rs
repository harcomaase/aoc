fn main() {
    let file_content = std::fs::read_to_string("../input/21/day8.txt").expect("read input file");

    let easy_digits: usize = file_content
        .lines()
        .map(|line| line.split(" | ").nth(1).unwrap())
        .map(|right_side| {
            right_side
                .split_ascii_whitespace()
                .filter(|signal| {
                    signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7
                })
                .count()
        })
        .sum();
    println!("easy digits: {}", easy_digits);
}
