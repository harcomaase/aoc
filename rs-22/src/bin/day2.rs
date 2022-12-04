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
    let hand2 = Hand::parse(chars[2]);

    let winner = match hand1 {
        Hand::Rock => match hand2 {
            Hand::Rock => 0,
            Hand::Paper => 2,
            Hand::Scissors => 1,
        },
        Hand::Paper => match hand2 {
            Hand::Rock => 1,
            Hand::Paper => 0,
            Hand::Scissors => 2,
        },
        Hand::Scissors => match hand2 {
            Hand::Rock => 2,
            Hand::Paper => 1,
            Hand::Scissors => 0,
        },
    };

    (winner, hand2)
}
