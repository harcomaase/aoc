use std::fs;

fn main() {
    let filename = "../input/19/day6.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut raw_orbits: Vec<(&str, &str)> = Vec::new();
    for line in file.lines() {
        if line.is_empty() {
            continue;
        }
        let x: Vec<&str> = line.split(")").collect();

        raw_orbits.push((x.get(0).unwrap(), x.get(1).unwrap()));
    }

    let root = "COM";

    let mut total = 0;

    for raw_orbit in raw_orbits {
        let mut it = raw_orbit;
        while it.0 != root {
            total += 1;

            it = find_orbit(raw_orbits, it.0).unwrap();
        }
    }

    println!("total: {}", total);
}

fn find_orbit(raw_orbits: &Vec<(&str, &str)>, orbiter: &str) -> Option<(&str, &str)> {
    for raw_orbit in raw_orbits {
        if raw_orbit.1 == orbiter {
            return raw_orbit;
        }
    }
    None
}
