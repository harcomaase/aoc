fn main() {
    let input = std::fs::read_to_string("./inputs/9.txt").unwrap();

    let output = solve(input);
    println!("{output}");
}

fn solve(input: String) -> usize {
    let diskmap: Vec<usize> = input
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| return c.to_digit(10).unwrap() as usize)
        .collect();
    let mut checksum = 0;

    // build disk Vec
    let mut disk = Vec::new();
    for i in (0..diskmap.len()).step_by(2) {
        let file = diskmap[i];
        let free = if i + 1 < diskmap.len() {
            diskmap[i + 1]
        } else {
            0
        };
        let index = i / 2;
        for _ in 0..file {
            disk.push(Some(index));
        }
        for _ in 0..free {
            disk.push(None);
        }
    }

    // compress
    let mut i = disk.len();
    'outer: loop {
        if i == 0 {
            break;
        }
        i -= 1;
        match disk[i] {
            Some(f) => {
                // find out how long the file is
                // (yes we knew that before)
                let mut i2 = i - 1;
                while disk[i2].is_some() {
                    if i2 == 0 {
                        break 'outer;
                    }
                    if disk[i2].unwrap() != f {
                        break;
                    }
                    i2 -= 1;
                }
                let filelen = i - i2;
                // now find a free spot
                // (could be optimised by starting from the first free spot)
                let mut p = 0;
                loop {
                    match disk[p] {
                        Some(_) => (),
                        None => {
                            // find out how long the free space is
                            // (yes we also knew that before)
                            let mut p2 = p + 1;
                            while p2 < disk.len() && disk[p2].is_none() {
                                p2 += 1;
                            }
                            let freelen = p2 - p;
                            if freelen >= filelen {
                                // file fits, move the file
                                for x in 0..filelen {
                                    disk[p + x] = disk[i - x];
                                    disk[i - x] = None;
                                }
                                break;
                            }
                            // file does not fit,
                            // move pointer by length of free space
                            p += freelen - 1;
                        }
                    }
                    p += 1;
                    if p > i || p >= disk.len() {
                        break;
                    }
                }

                // move pointer by length of file
                i -= filelen - 1;
            }
            None => {
                continue;
            }
        }
    }

    for i in 0..disk.len() {
        match disk[i] {
            Some(f) => {
                checksum += i * f;
            }
            None => {
                continue;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input1() {
        let input = std::fs::read_to_string("./inputs/9-t1.txt").unwrap();

        let output = solve(input);
        assert_eq!(2858, output);
    }
}
