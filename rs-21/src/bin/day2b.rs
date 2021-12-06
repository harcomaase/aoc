use regex::Regex;

fn main() {
    let file_content = std::fs::read_to_string("../input/21/day2.txt").expect("read input file");

    let regex = Regex::new("([a-z]+) ([0-9]+)").expect("create regex");

    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    file_content
        .lines()
        .map(|line| regex.captures(line).expect("parsing input line"))
        .map(|capture| {
            (
                capture.get(1).expect("capture group 1").as_str(),
                capture
                    .get(2)
                    .expect("capture group 2")
                    .as_str()
                    .parse::<i32>()
                    .expect("parsing number"),
            )
        })
        .for_each(|(direction, value)| match direction {
            "forward" => {
                x += value;
                z += value * aim;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("invalid direction: {}", direction),
        });

    println!("{}", x * z);
}
