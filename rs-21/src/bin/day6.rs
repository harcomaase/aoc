fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day6.txt").expect("read input file");

    let mut lanternfish: Vec<u8> = file_content
        .split(",")
        .map(|line| line.parse::<u8>().expect("parsing input file"))
        .collect();
    let fresh_counter = 8;
    let reset_counter = 6;

    let days = 80;

    for _day in 0..days {
        let mut spawn_counter = 0;
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish[i] = reset_counter;
                spawn_counter += 1;
            } else {
                lanternfish[i] -= 1;
            }
        }
        for _i in 0..spawn_counter {
            lanternfish.push(fresh_counter);
        }
        // println!("After {} days: {:?}", _day, lanternfish);
    }
    println!("number of lanternfish: {}", lanternfish.len());
}
