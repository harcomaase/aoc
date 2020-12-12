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
    let (mut wp_x, mut wp_y) = (10, 1);

    for evasive_action in evasive_actions {
        let action = evasive_action.0;
        let value = evasive_action.1;

        println!("evasive_action: {:?}", evasive_action);

        match action {
            "N" => wp_y += value,
            "S" => wp_y -= value,
            "E" => wp_x += value,
            "W" => wp_x -= value,
            "L" | "R" => {
                let tmp = update_waypoint((wp_x, wp_y), (x, y), action, value);
                wp_x = tmp.0;
                wp_y = tmp.1;
            }
            "F" => {
                let (tmp_x, tmp_y) = (x, y);
                x += value * (wp_x - x);
                y += value * (wp_y - y);
                wp_x += value * (wp_x - tmp_x);
                wp_y += value * (wp_y - tmp_y);
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

fn update_waypoint(
    (wp_x, wp_y): (i64, i64),
    (x, y): (i64, i64),
    action: &str,
    value: i64,
) -> (i64, i64) {
    let effective_degrees = value % 360;

    match effective_degrees {
        0 => return (wp_x, wp_y),
        90 => return turn_ninety_degrees((wp_x, wp_y), (x, y), action),
        180 => return (x - (wp_x - x), y - (wp_y - y)),
        270 => {
            return turn_ninety_degrees(
                (wp_x, wp_y),
                (x, y),
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

fn turn_ninety_degrees((wp_x, wp_y): (i64, i64), (x, y): (i64, i64), action: &str) -> (i64, i64) {
    match action {
        "L" => return (x - (wp_y - y), y + (wp_x - x)),
        "R" => return (x + (wp_y - y), y - (wp_x - x)),
        other => panic!("invalid action: {}", other),
    }
}
