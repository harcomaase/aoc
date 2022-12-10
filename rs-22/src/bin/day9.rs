use std::fs;

#[derive(PartialEq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("../input/22/day9.txt").unwrap();

    let mut pos_head = Pos { x: 0, y: 0 };
    let mut pos_tail = Pos { x: 0, y: 0 };

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
            pos_head.x += dx;
            pos_head.y += dy;

            if !adjacent(&pos_head, &pos_tail) {
                if pos_head.x == pos_tail.x {
                    if pos_head.y > pos_tail.y {
                        pos_tail.y += 1;
                    } else {
                        pos_tail.y -= 1;
                    }
                } else if pos_head.y == pos_tail.y {
                    if pos_head.x > pos_tail.x {
                        pos_tail.x += 1;
                    } else {
                        pos_tail.x -= 1;
                    }
                } else {
                    if pos_head.x > pos_tail.x {
                        pos_tail.x += 1;
                    } else {
                        pos_tail.x -= 1;
                    }
                    if pos_head.y > pos_tail.y {
                        pos_tail.y += 1;
                    } else {
                        pos_tail.y -= 1;
                    }
                }
            }
            if !adjacent(&pos_head, &pos_tail) {
                panic!("head and tail not adjacent after moving!");
            }
            if !positions_tail_visitited.contains(&pos_tail) {
                positions_tail_visitited.push(pos_tail.clone());
            }

            // println!(
            //     "head ({}|{}), tail ({}|{})",
            //     pos_head.x, pos_head.y, pos_tail.x, pos_tail.y
            // );
        }
    }

    println!(
        "tail visited {} distinct positions",
        positions_tail_visitited.len()
    );
}

fn adjacent(pos_head: &Pos, pos_tail: &Pos) -> bool {
    (pos_head.x - 1..=pos_head.x + 1).contains(&pos_tail.x)
        && (pos_head.y - 1..=pos_head.y + 1).contains(&pos_tail.y)
}
