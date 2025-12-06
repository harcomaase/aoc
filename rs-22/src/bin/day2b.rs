use std::fs;

fn main() {
    let input = fs::read_to_string("../input/22/day2.txt").unwrap();

    let total_score: i32 = input
        .lines()
        .map(|line| {
            let (winner, hand) = winner(line);
            let mut score = 0;
            score += match winner {
                0 => 3,
                1 => 0,
                2 => 6,
                _ => panic!(),
            };
            score += match hand {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };
            score
        })
        .sum();

    println!("{}", total_score);
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn parse(c: char) -> Hand {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

fn winner(line: &str) -> (u8, Hand) {
    let chars: Vec<char> = line.chars().collect();
    let hand1 = Hand::parse(chars[0]);
    let expected_result = chars[2];

    let winner = match expected_result {
        'X' => 1,
        'Y' => 0,
        'Z' => 2,
        _ => panic!(),
    };

    let hand2 = match hand1 {
        Hand::Rock => match expected_result {
            'X' => Hand::Scissors,
            'Y' => Hand::Rock,
            'Z' => Hand::Paper,
            _ => panic!(),
        },
        Hand::Paper => match expected_result {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!(),
        },
        Hand::Scissors => match expected_result {
            'X' => Hand::Paper,
            'Y' => Hand::Scissors,
            'Z' => Hand::Rock,
            _ => panic!(),
        },
    };

    (winner, hand2)
}
