use regex::Regex;

struct Pipe {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

fn main() {
    let file_content =
        std::fs::read_to_string("../input/21/day5.txt").expect("read input file");

    let regex = Regex::new("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").expect("create regex");

    let pipes: Vec<Pipe> = file_content
        .lines()
        .map(|line| regex.captures(line).expect("parsing input"))
        .map(|cap| Pipe {
            x1: cap
                .get(1)
                .expect("capture group 1")
                .as_str()
                .parse::<i64>()
                .expect("parsing input"),
            y1: cap
                .get(2)
                .expect("capture group 2")
                .as_str()
                .parse::<i64>()
                .expect("parsing input"),
            x2: cap
                .get(3)
                .expect("capture group 3")
                .as_str()
                .parse::<i64>()
                .expect("parsing input"),
            y2: cap
                .get(4)
                .expect("capture group 4")
                .as_str()
                .parse::<i64>()
                .expect("parsing input"),
        })
        // .filter(|pipe| pipe.x1 == pipe.x2 || pipe.y1 == pipe.y2)
        .collect();
    let (max_x, max_y) = get_max(&pipes);

    // n-dimensional Vector of Vectors, initialised with value 0
    let mut diagram: Vec<Vec<u8>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for pipe in pipes {
        if pipe.x1 == pipe.x2 {
            //vertical
            for y in i64::min(pipe.y1, pipe.y2)..=i64::max(pipe.y1, pipe.y2) {
                diagram[y as usize][pipe.x1 as usize] += 1;
            }
        } else if pipe.y1 == pipe.y2 {
            //horizontal
            for x in i64::min(pipe.x1, pipe.x2)..=i64::max(pipe.x1, pipe.x2) {
                diagram[pipe.y1 as usize][x as usize] += 1;
            }
        } else {
            //diagonal
            let (vector_x, vector_y) = (pipe.x2 - pipe.x1, pipe.y2 - pipe.y1);
            let vector_len = i64::max(pipe.x1, pipe.x2) - i64::min(pipe.x1, pipe.x2);
            let (step_x, step_y) = (vector_x / vector_len, vector_y / vector_len);
            for i in 0..=vector_len {
                diagram[(pipe.y1 + i * step_y) as usize][(pipe.x1 + i * step_x) as usize] += 1;
            }
        }
    }

    let overlaps = diagram
        .iter()
        .map(|row| row.iter().filter(|value| **value > 1).count())
        .sum::<usize>();
    println!("overlaps: {}", overlaps);
}

fn get_max(pipes: &Vec<Pipe>) -> (i64, i64) {
    let mut max_x = 0;
    let mut max_y = 0;

    for pipe in pipes {
        if pipe.x1 > max_x {
            max_x = pipe.x1;
        }
        if pipe.x2 > max_x {
            max_x = pipe.x2;
        }
        if pipe.y1 > max_y {
            max_y = pipe.y1;
        }
        if pipe.y2 > max_y {
            max_y = pipe.y2;
        }
    }
    (max_x, max_y)
}
