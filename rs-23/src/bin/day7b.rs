use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
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

        let joker_quantity = *map.get(&'J').unwrap_or(&0);

        match map.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => {
                if joker_quantity > 0 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            3 => {
                if joker_quantity == 3 {
                    match map.values().find(|v| **v == 2) {
                        Some(_) => HandType::FiveOfAKind,
                        None => HandType::FourOfAKind,
                    }
                } else if joker_quantity == 2 {
                    HandType::FiveOfAKind
                } else if joker_quantity == 1 {
                    HandType::FourOfAKind
                } else {
                    match map.values().find(|v| **v == 2) {
                        Some(_) => HandType::FullHouse,
                        None => HandType::ThreeOfAKind,
                    }
                }
            }
            2 => {
                if joker_quantity == 2 {
                    if map.values().filter(|v| **v == 2).count() == 2 {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if joker_quantity == 1 {
                    if map.values().filter(|v| **v == 2).count() == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    if map.values().filter(|v| **v == 2).count() == 2 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
            }
            1 => {
                if joker_quantity == 1 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
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
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            'J' => Self::Jack,
            _ => panic!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let o1 = HandType::from(self);
        let o2 = HandType::from(other);

        Some(match o1.cmp(&o2) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                // we need to compare hands card by card
                for i in 0..self.cards.len() {
                    match Card::from(self.cards[i]).cmp(&Card::from(other.cards[i])) {
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
    let input = fs::read_to_string("../input/23/day7.txt").unwrap();

    let mut hand_with_bids: Vec<(Hand, u64)> = input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut split| {
            (
                Hand {
                    cards: split.next().unwrap().chars().collect(),
                },
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    hand_with_bids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let total_winnings: u64 = hand_with_bids
        .iter()
        .enumerate()
        .map(|(i, hwb)| (i as u64 + 1) * hwb.1)
        .sum();
    println!("{total_winnings}");
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

        assert!(Card::from('K') > Card::from('T'));

        let five_of_a_kind = &Hand {
            cards: vec!['2', '2', '2', '2', '2'],
        };
        let four_of_a_kind = &Hand {
            cards: vec!['A', '2', '2', '2', '2'],
        };
        let three_of_a_kind = &Hand {
            cards: vec!['A', 'K', '2', '2', '2'],
        };
        let three_of_a_kind2 = &Hand {
            cards: vec!['A', '2', '2', 'K', '2'],
        };
        let full_house = &Hand {
            cards: vec!['A', 'A', '2', '2', '2'],
        };
        let full_house2 = &Hand {
            cards: vec!['A', 'A', 'A', '2', '2'],
        };
        let two_pair = &Hand {
            cards: vec!['2', '2', '3', '3', '4'],
        };
        let two_pair2 = &Hand {
            cards: vec!['A', '2', 'A', '2', '4'],
        };
        let one_pair = &Hand {
            cards: vec!['A', 'A', '2', '3', '4'],
        };
        let one_pair2 = &Hand {
            cards: vec!['A', 'K', '2', '3', 'A'],
        };
        let one_pair3 = &Hand {
            cards: vec!['K', 'A', '2', '2', '4'],
        };
        let high_card = &Hand {
            cards: vec!['2', '3', '4', '5', '6'],
        };

        assert_eq!(HandType::FiveOfAKind, HandType::from(five_of_a_kind));
        assert_eq!(HandType::FourOfAKind, HandType::from(four_of_a_kind));
        assert_eq!(HandType::FullHouse, HandType::from(full_house));
        assert_eq!(HandType::FullHouse, HandType::from(full_house2));
        assert_eq!(HandType::ThreeOfAKind, HandType::from(three_of_a_kind));
        assert_eq!(HandType::ThreeOfAKind, HandType::from(three_of_a_kind2));
        assert_eq!(HandType::TwoPair, HandType::from(two_pair));
        assert_eq!(HandType::TwoPair, HandType::from(two_pair2));
        assert_eq!(HandType::OnePair, HandType::from(one_pair));
        assert_eq!(HandType::OnePair, HandType::from(one_pair2));
        assert_eq!(HandType::OnePair, HandType::from(one_pair3));
        assert_eq!(HandType::HighCard, HandType::from(high_card));

        assert_eq!(
            HandType::from(five_of_a_kind),
            HandType::from(five_of_a_kind)
        );

        assert!(
            Hand {
                cards: vec!['T', 'T', 'T', 'T', '7']
            } > Hand {
                cards: vec!['T', 'T', 'T', '7', '7']
            }
        );
        assert!(
            Hand {
                cards: vec!['T', 'T', 'T', '7', '6']
            } > Hand {
                cards: vec!['T', 'T', 'T', '6', '7']
            }
        );
        assert!(
            Hand {
                cards: vec!['T', 'T', 'T', '7', '8']
            } < Hand {
                cards: vec!['T', 'T', 'T', '7', '9']
            }
        );
        assert!(
            Hand {
                cards: vec!['T', 'T', 'T', '7', '9']
            } == Hand {
                cards: vec!['T', 'T', 'T', '7', '9']
            }
        );

        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'A', 'A', 'J']
            })
        );
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'A', 'J', 'J']
            })
        );
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'J', 'J', 'J']
            })
        );
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'J', 'J', 'J', 'J']
            })
        );
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&Hand {
                cards: vec!['J', 'J', 'J', 'J', 'J']
            })
        );

        assert_eq!(
            HandType::FourOfAKind,
            HandType::from(&Hand {
                cards: vec!['J', 'J', 'J', 'A', 'Q']
            })
        );
        assert_eq!(
            HandType::FourOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'J', 'J', 'Q']
            })
            );

        assert_eq!(
            HandType::FullHouse,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'A', 'J', 'Q']
            })
        );
        assert_eq!(
            HandType::FullHouse,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'J', 'Q', 'Q']
            })
        );

        assert_eq!(
            HandType::ThreeOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'K', 'J', 'J', 'Q']
            })
        );
        assert_eq!(
            HandType::ThreeOfAKind,
            HandType::from(&Hand {
                cards: vec!['A', 'A', 'J', 'K', 'Q']
            })
        );

        assert_eq!(
            HandType::OnePair,
            HandType::from(&Hand {
                cards: vec!['A', 'Q', 'J', 'K', 'T']
            })
        );
    }
}
