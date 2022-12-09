use std::fs;

#[derive(PartialEq, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("../input/22/day9.txt").unwrap();

    let length: usize = 10;
    let mut pos = Vec::with_capacity(length);
    for _i in 0..length {
        pos.push(Pos { x: 0, y: 0 });
    }

    let mut positions_tail_visitited = Vec::new();

    for line in input.lines() {
        let steps = &line[2..].parse::<i32>().unwrap();
        let (dx, dy) = match &line[..1] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("unparseable line: {}", line),
        };

        for _i in 0..*steps {
            pos[0].x += dx;
            pos[0].y += dy;

            for tail in 1..length {
                let (tail_dx, tail_dy) = calculate_delta(&pos, tail);
                pos[tail].x += tail_dx;
                pos[tail].y += tail_dy;
                if !adjacent(&pos[tail], &pos[tail - 1]) {
                    panic!("head and tail not adjacent after moving!");
                }
            }
            let pos_tail = &pos[length - 1];
            if !positions_tail_visitited.contains(pos_tail) {
                positions_tail_visitited.push(pos_tail.clone());
            }
        }
    }

    println!(
        "tail visited {} distinct positions",
        positions_tail_visitited.len()
    );
}

fn calculate_delta(pos: &Vec<Pos>, tail: usize) -> (i32, i32) {
    let pos_head = &pos[tail - 1];
    let pos_tail = &pos[tail];
    if !adjacent(&pos_head, &pos_tail) {
        let (tail_dx, tail_dy) = if pos_head.x == pos_tail.x {
            if pos_head.y > pos_tail.y {
                (0, 1)
            } else {
                (0, -1)
            }
        } else if pos_head.y == pos_tail.y {
            if pos_head.x > pos_tail.x {
                (1, 0)
            } else {
                (-1, 0)
            }
        } else {
            let tail_dx = if pos_head.x > pos_tail.x { 1 } else { -1 };
            let tail_dy = if pos_head.y > pos_tail.y { 1 } else { -1 };
            (tail_dx, tail_dy)
        };
        return (tail_dx, tail_dy);
    }
    (0, 0)
}

fn adjacent(pos_head: &Pos, pos_tail: &Pos) -> bool {
    (pos_head.x - 1..=pos_head.x + 1).contains(&pos_tail.x)
        && (pos_head.y - 1..=pos_head.y + 1).contains(&pos_tail.y)
}
