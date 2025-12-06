use std::fs;

fn main() {
    let filename = "../input/20/day4.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let passports: Vec<&str> = file.split("\n\n").collect();

    let mandatory_fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    let mut valid_passports = 0;

    for passport in passports {
        let valid_passport = mandatory_fields
            .iter()
            .all(|&field| passport.contains(field));

        if valid_passport {
            valid_passports += 1;
        }
    }

    println!("valid passports: {}", valid_passports);
}
