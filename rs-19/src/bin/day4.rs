
fn main() {
    let a = 146810;
    let b = 612564;

    let mut count = 0;

    for i in a..b {
        let mut last_digit = digit_at(i, 0);
        let mut valid_adj = false;
        let mut valid_inc = true;
        for n in 1..6 {
            let digit = digit_at(i, n);
            if !valid_adj && digit == last_digit {
                valid_adj = true;
            }
            if last_digit > digit {
                valid_inc = false;
            }
            last_digit = digit;
        }
        if valid_adj && valid_inc {
            count += 1;
        }
    }

    println!("{}", count);
}

fn digit_at(number: i32, pos: i32) -> i32 {
    let mut result = number;
    for _ in 1..(6 - pos) {
        result /= 10;
    }
    result % 10
}
