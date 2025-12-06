use std::fs;

fn main() {
    let filename = "../input/20/day13.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");
    let raw_input: Vec<&str> = file.split("\n").collect();
    let bus_ids: Vec<&str> = raw_input.get(1).expect("valid index").split(",").collect();
    let mut input: Vec<(usize, usize)> = vec![];
    for i in 0..bus_ids.len() {
        let bus_id = bus_ids.get(i).expect("valid index");
        match bus_id {
            &"x" => continue,
            _ => (),
        }
        let id = bus_id.parse::<usize>().expect("parsable number");
        input.push((id, i));
    }

    input.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", input);

    let soonest_timestamp = find_soonest_timestamp(&input);

    println!("{}", soonest_timestamp);
}

fn find_soonest_timestamp(input: &Vec<(usize, usize)>) -> usize {
    let mut soonest_timestamp = 1;
    let mut multi = 1;

    for departure in input {
        //(bus_id, offset)
        while (soonest_timestamp + departure.1) % departure.0 != 0 {
            soonest_timestamp += multi;
        }
        multi *= departure.0;
    }
    soonest_timestamp
}
