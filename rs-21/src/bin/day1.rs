fn main() {
    let file_content = std::fs::read_to_string("../input/21/day1.txt").expect("read input file");

    let input: Vec<i32> = file_content
        .lines()
        .map(|line| line.parse::<i32>().expect("parsing input file"))
        .collect();

    let mut increments = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increments += 1;
        }
    }
    println!("{}", increments);
}
