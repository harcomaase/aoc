use std::{collections::HashMap, fs};

#[derive(PartialEq)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard(Card),
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut map = HashMap::new();
        for c in &value.cards {
            *map.entry(*c).or_insert(0) += 1;
        }

        match map.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match map.values().find(|v| **v == 2) {
                Some(_) => HandType::FullHouse,
                None => HandType::ThreeOfAKind,
            },
            2 => {
                if map.values().filter(|v| **v == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => {
                let highest_card = map.keys().map(|c| Card::from(*c)).max().unwrap();
                HandType::HighCard(highest_card)
            }
            _ => panic!(),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let o1 = HandType::from(self);
        let o2 = HandType::from(other);

        Some(match o2.cmp(&o1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                // we need to compare hands card by card
                for i in 0..self.cards.len() {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                    }
                }
                return Some(std::cmp::Ordering::Equal);
            }
        })
    }
}

fn main() {
    let _input = fs::read_to_string("../input/23/day7test.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison() {
        assert_eq!(Card::from('2'), Card::from('2'));
        assert_ne!(Card::from('2'), Card::from('3'));

        assert!(Card::from('2') < Card::from('3'));
        assert!(Card::from('4') > Card::from('3'));

        assert_eq!(
            HandType::from(&Hand {
                cards: vec!['2', '2', '2', '2', '2']
            }),
            HandType::from(&Hand {
                cards: vec!['2', '2', '2', '2', '2']
            })
        );
    }
}
