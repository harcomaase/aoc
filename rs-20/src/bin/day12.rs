use regex::Regex;
use std::fs;

fn main() {
    let filename = "../input/20/day12.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let regex = Regex::new(r"^([NSEWLRF])([0-9]+)").expect("valid regex");

    let evasive_actions: Vec<(&str, i64)> = file
        .split("\n")
        .map(|s| regex.captures(s).expect("successful capture"))
        .map(|capture| {
            (
                capture.get(1).expect("valid group 1").as_str(),
                capture
                    .get(2)
                    .expect("valid group 2")
                    .as_str()
                    .parse::<i64>()
                    .expect("valid number"),
            )
        })
        .collect();

    let (mut x, mut y) = (0, 0);
    let (mut dir_x, mut dir_y) = (1, 0); //east

    for evasive_action in evasive_actions {
        let action = evasive_action.0;
        let value = evasive_action.1;

        println!("evasive_action: {:?}", evasive_action);

        match action {
            "N" => y += value,
            "S" => y -= value,
            "E" => x += value,
            "W" => x -= value,
            "L" | "R" => {
                let tmp = update_direction((dir_x, dir_y), action, value);
                dir_x = tmp.0;
                dir_y = tmp.1;
            }
            "F" => {
                x += value * dir_x;
                y += value * dir_y;
            }
            other => panic!("invalid action: {}", other),
        }

        println!("updated coords: ({}|{})", x, y);
    }

    println!(
        "final coords: ({}|{}), manhattan distance: {}",
        x,
        y,
        x.abs() + y.abs()
    );
}

fn update_direction((dir_x, dir_y): (i64, i64), action: &str, value: i64) -> (i64, i64) {
    let effective_degrees = value % 360;

    match effective_degrees {
        0 => return (dir_x, dir_y),
        90 => return turn_ninety_degrees((dir_x, dir_y), action),
        180 => return (dir_x * -1, dir_y * -1),
        270 => {
            return turn_ninety_degrees(
                (dir_x, dir_y),
                match action {
                    "L" => "R",
                    "R" => "L",
                    _ => "bla",
                },
            )
        }
        other => panic!("invalid degrees value: {}", other),
    }
}

fn turn_ninety_degrees((dir_x, dir_y): (i64, i64), action: &str) -> (i64, i64) {
    match action {
        "L" => match (dir_x, dir_y) {
            (-1, 0) => return (0, -1),
            (0, -1) => return (1, 0),
            (1, 0) => return (0, 1),
            (0, 1) => return (-1, 0),
            other => panic!("invalid direction: {:?}", other),
        },
        "R" => match (dir_x, dir_y) {
            (-1, 0) => return (0, 1),
            (0, 1) => return (1, 0),
            (1, 0) => return (0, -1),
            (0, -1) => return (-1, 0),
            other => panic!("invalid direction: {:?}", other),
        },
        other => panic!("invalid action: {}", other),
    }
}
