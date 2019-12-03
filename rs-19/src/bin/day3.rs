use std::cmp;
use std::fs;

struct Point {
    x: i32,
    y: i32,
}

struct Wire {
    start: Point,
    end: Point,
}

fn main() {
    let filename = "../input/19/day3.txt";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut paths: Vec<Vec<Wire>> = Vec::new();
    for file_line in file.lines() {
        if file_line.is_empty() {
            continue;
        }
        let input: Vec<&str> = file_line.split(",").collect();
        let mut path: Vec<Wire> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for movement in input {
            let p1 = Point { x, y };
            let (direction, steps) = movement.split_at(1);
            let steps = steps.parse::<i32>().unwrap();
            match direction {
                "U" => y += steps,
                "D" => y -= steps,
                "L" => x -= steps,
                "R" => x += steps,
                _ => panic!("unknown direction"),
            }
            path.push(Wire {
                start: p1,
                end: Point { x, y },
            });
        }
        println!("path: {}", path.len());
        paths.push(path);
    }

    println!("paths: {}", paths.len());

    let mut intersections: Vec<Point> = Vec::new();
    for i1 in 0..paths.len() {
        let path1 = paths.get(i1).unwrap();
        for i2 in 0..paths.len() {
            let path2 = paths.get(i2).unwrap();
            if i1 == i2 {
                break;
            }

            for w1 in path1 {
                for w2 in path2 {
                    match intersect(&w1, &w2) {
                        Some(p) => {
                            let x = p.x;
                            let y = p.y;
                            println!("found intersection at [{},{}]", x, y);
                            intersections.push(Point { x, y });
                        }
                        None => (),
                    }
                }
            }
        }
    }

    println!("intersections: {}", intersections.len());

    let mut min_dist = 0;

    for p in intersections {
        let manhattan_distance = abs(p.x) + abs(p.y);
        if min_dist == 0 || manhattan_distance < min_dist {
            min_dist = manhattan_distance;
        }
    }

    println!("min_manhattan_dist: {}", min_dist);
}

fn intersect(w1: &Wire, w2: &Wire) -> Option<Point> {
    let w1_vertical = w1.start.x == w1.end.x;
    let w2_vertical = w2.start.x == w2.end.x;

    if w1_vertical && w2_vertical {
        if w1.start.x != w2.start.x
            || !between(w1.start.y, w2.start.y, w2.end.y)
                && !between(w1.end.y, w2.start.y, w2.end.y)
            || w1.start.x == 0 && w2.start.x == 0 && w1.start.y == 0 && w2.start.y == 0
        {
            return Option::None;
        } else {
            panic!("parallel intersecting lines detected!");
        }
    }
    if !w1_vertical && !w2_vertical {
        if w1.start.y != w2.start.y
            || !between(w1.start.x, w2.start.x, w2.end.x)
                && !between(w1.end.x, w2.start.x, w2.end.x)
            || w1.start.x == 0 && w2.start.x == 0 && w1.start.y == 0 && w2.start.y == 0
        {
            return Option::None;
        } else {
            panic!("parallel intersecting lines detected!");
        }
    }

    if w1_vertical {
        let w1x = w1.start.x;
        let w2y = w2.start.y;
        if between(w1x, w2.start.x, w2.end.x) && between(w2y, w1.start.y, w1.end.y) {
            return Option::Some(Point { x: w1x, y: w2y });
        }
    } else {
        let w2x = w2.start.x;
        let w1y = w1.start.y;
        if between(w2x, w1.start.x, w1.end.x) && between(w1y, w2.start.y, w2.end.y) {
            return Option::Some(Point {
                x: w2x,
                y: w1.start.y,
            });
        }
    }

    Option::None
}

fn between(value: i32, a: i32, b: i32) -> bool {
    return value >= cmp::min(a, b) && value <= cmp::max(a, b);
}

fn abs(val: i32) -> i32 {
    if val < 0 {
        return -val;
    }
    val
}
