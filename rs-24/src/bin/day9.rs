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
    let mut p = 0;
    for i in (0..disk.len()).rev() {
        if p >= i {
            break;
        }
        match disk[i] {
            Some(f) => {
                while disk[p].is_some() {
                    p += 1;
                }
                if p >= i {
                    break;
                }
                // move i to p
                disk[p] = Some(f);
                disk[i] = None;
            }
            None => {
                continue;
            }
        }
    }

    for i in 0.. {
        match disk[i] {
            Some(f) => {
                checksum += i * f;
            }
            None => {
                break;
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
        assert_eq!(1928, output);
    }
}
