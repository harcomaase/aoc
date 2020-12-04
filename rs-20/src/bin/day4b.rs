use regex::Regex;
use std::fs;

fn main() {
    let filename = "../input/20/day4.txt";
    let file = fs::read_to_string(filename).expect("successfully reading the input");

    let passports: Vec<&str> = file.split("\n\n").collect();

    let mandatory_fields = vec![
        Regex::new("byr:(19[2-9][0-9]|200[012])").expect("valid regex"),
        Regex::new("iyr:20(1[0-9]|20)").expect("valid regex"),
        Regex::new("eyr:20(2[0-9]|30)").expect("valid regex"),
        Regex::new("hgt:(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)").expect("valid regex"),
        Regex::new("hcl:#[0-9a-f]{6}").expect("valid regex"),
        Regex::new("ecl:(amb|blu|brn|gry|grn|hzl|oth)").expect("valid regex"),
        Regex::new("pid:[0-9]{9}").expect("valid regex"),
    ];
    let mut valid_passports = 0;

    for passport in passports {
        let valid_passport = mandatory_fields
            .iter()
            .all(|field| field.is_match(passport));

        if valid_passport {
            valid_passports += 1;
            println!("{}\n", passport);
        }
    }

    println!("valid passports: {}", valid_passports);
}
