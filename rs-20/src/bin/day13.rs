use std::fs;

fn main() {
    let filename = "../input/20/day13.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let input: Vec<&str> = file.split("\n").collect();

    let earliest_departure: u64 = input
        .get(0)
        .expect("valid index")
        .parse()
        .expect("valid unsigned number");

    let mut min_wait_times: Vec<(u64, u64)> = input
        .get(1)
        .expect("valid index")
        .split(",")
        .filter(|s| match s {
            &"x" => false,
            _ => true,
        })
        .map(|s| s.parse::<u64>().expect("parsable number"))
        .map(|bus_id| (bus_id, bus_id - earliest_departure % bus_id))
        .collect();

    min_wait_times.sort_by(|a, b| a.1.cmp(&b.1));

    for min_wait_time in min_wait_times {
        println!(
            "{:?} -> {}",
            min_wait_time,
            min_wait_time.0 * min_wait_time.1
        );
    }
}
