fn main() {
    let input = std::fs::read_to_string("./inputs/2.txt").unwrap();

    let mut total = 0;
    for report in input.lines() {
        let mut levels = report
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap());
        let mut last = levels.next().unwrap();
        let mut safe = true;
        let mut ord = Ord::UNSURE;
        for level in levels {
            let diff = i32::abs(level - last);
            if diff > 3 || diff < 1 {
                safe = false;
                break;
            }
            match last - level {
                ..=-1 => {
                    if ord == Ord::UNSURE {
                        ord = Ord::ASC;
                    } else if ord == Ord::DESC {
                        safe = false;
                        break;
                    }
                }
                0 => (), // can not happen
                1.. => {
                    if ord == Ord::UNSURE {
                        ord = Ord::DESC;
                    } else if ord == Ord::ASC {
                        safe = false;
                        break;
                    }
                }
            }
            last = level;
        }
        if safe {
            total += 1;
        }
    }
    println!("{total}");
}

#[derive(PartialEq)]
enum Ord {
    UNSURE,
    ASC,
    DESC,
}
