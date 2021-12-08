fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day6.txt").expect("read input file");

    let mut lanternfish: [u64; 9] = [0; 9];

    file_content
        .split(",")
        .map(|line| line.parse::<usize>().expect("parsing input file"))
        .for_each(|fish| lanternfish[fish] += 1);
    let fresh_counter = 8;
    let reset_counter = 6;

    let days = 256;

    for day in 0..days {
        let spawn_counter = lanternfish[0];
        for i in 1..lanternfish.len() {
            lanternfish[i - 1] = lanternfish[i];
        }
        lanternfish[reset_counter] += spawn_counter;
        lanternfish[fresh_counter] = spawn_counter;
        println!("After {} days: {} fish", day, sum(lanternfish));
    }
    println!("number of lanternfish: {}", sum(lanternfish));
}

fn sum(arr: [u64; 9]) -> u64 {
    let mut sum = 0;
    for value in arr {
        sum += value;
    }
    sum
}
