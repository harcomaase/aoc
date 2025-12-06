use std::fs;

fn main() {
    //TODO: does not work yet
    let input = fs::read_to_string("inputs/day02.txt").unwrap();

    let mut result = 0;
    for range in input.split(',') {
        let mut range_split = range.split('-');
        let start: u64 = range_split.next().unwrap().trim().parse().unwrap();
        let end: u64 = range_split.next().unwrap().trim().parse().unwrap();

        for i in start..=end {
            let s = i.to_string();
            // single digit numbers are not valid
            if s.len() == 1 {
                continue;
            }
            // check in how many parts the given number can be broken down
            let mut denominators = vec![1];
            for d in 2..=(s.len() / 2) {
                if s.len() % d == 0 {
                    denominators.push(d);
                }
            }
            for denominator in denominators {
                let parts_count = s.len() / denominator;
                let part_length = s.len() / parts_count;
                let mut parts = Vec::with_capacity(s.len());

                // split the number into parts
                for index in 0..parts_count {
                    parts.push(&s[(index * part_length)..((index + 1) * part_length)]);
                }

                // compare the parts
                if parts.windows(2).all(|window| window[0].eq(window[1])) {
                    result += i;
                    //println!("{s}\t{parts:?}");
                    break;
                }
            }
        }
    }

    println!("{result}");
}
