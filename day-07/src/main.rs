use lib::*;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    None,
    OnePair,
    TowPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<char>,
    pairs: Vec<(char, usize)>,
    bet: usize,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let (cards, bet) = line.split_once(' ').unwrap();

        let mut pairs: HashMap<char, usize> = HashMap::new();
        for c in cards.chars() {
            *pairs.entry(c).or_default() += 1;
        }

        Self {
            cards: cards.chars().collect(),
            pairs: pairs
                .iter()
                .filter(|(_, &n)| n > 1)
                .map(|(c, n)| (*c, *n))
                .collect(),
            bet: bet.parse().unwrap(),
        }
    }

    fn typ(&self) -> Type {
        if self.pairs.len() == 1 {
            return match self.pairs[0].1 {
                5 => Type::FiveOAK,
                4 => Type::FourOAK,
                3 => Type::ThreeOAK,
                2 => Type::OnePair,
                _ => Type::None,
            };
        }

        if self.pairs.len() == 2 {
            return match (self.pairs[0].1, self.pairs[1].1) {
                (2, 3) | (3, 2) => Type::FullHouse,
                (2, 2) => Type::TowPair,
                _ => Type::None,
            };
        }

        Type::None
    }

    fn card_value(card: char) -> usize {
        match card {
            'A' => 104,
            'K' => 103,
            'Q' => 102,
            'J' => 101,
            'T' => 100,
            x => x as usize,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.typ().cmp(&other.typ());
        if ord != Ordering::Equal {
            return ord;
        }

        for (&s, &o) in self.cards.iter().zip(&other.cards) {
            let ord = Self::card_value(s).cmp(&Self::card_value(o));
            if ord != Ordering::Equal {
                return ord;
            }
        }

        panic!("There should not be any equal card sets!");
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandWithJoker(Hand);

impl HandWithJoker {
    fn parse(line: &str) -> Self {
        let mut hand = Hand::parse(line);

        let joker = hand.cards.iter().filter(|c| **c == 'J').count();

        match joker {
            4 => hand.pairs = vec![('J', 5)],
            3 => match hand.pairs.len() {
                2 => hand.pairs = vec![('J', 5)],
                1 => hand.pairs = vec![('J', 4)],
                _ => panic!("should not happen; 3 jokers"),
            },
            2 => match hand.pairs.len() {
                2 => {
                    if hand.pairs[0].1 == hand.pairs[1].1 {
                        hand.pairs = vec![('J', 4)];
                    } else {
                        hand.pairs = vec![('J', 5)];
                    }
                }
                1 => hand.pairs = vec![('J', 3)],
                _ => panic!("should not happen; 2 jokers"),
            },
            1 => match hand.pairs.len() {
                2 => hand.pairs = vec![('J', 3), ('J', 2)],
                1 => match hand.pairs[0].1 {
                    4 => hand.pairs = vec![('J', 5)],
                    2 => hand.pairs = vec![('J', 3)],
                    3 => hand.pairs = vec![('J', 4)],
                    _ => panic!("should not happen; pairs can not be empty"),
                },
                0 => hand.pairs = vec![('J', 2)],
                _ => panic!("should not happen; too much pairs"),
            },
            _ => {}
        }

        Self(hand)
    }

    fn typ(&self) -> Type {
        match self.0.pairs.len() {
            2 => match (self.0.pairs[0].1, self.0.pairs[1].1) {
                (2, 3) | (3, 2) => Type::FullHouse,
                (2, 2) => Type::TowPair,
                _ => Type::None,
            },
            1 => match self.0.pairs[0].1 {
                5 => Type::FiveOAK,
                4 => Type::FourOAK,
                3 => Type::ThreeOAK,
                2 => Type::OnePair,
                _ => Type::None,
            },
            _ => Type::None,
        }
    }

    fn card_value(card: char) -> usize {
        match card {
            'A' => 104,
            'K' => 103,
            'Q' => 102,
            'T' => 100,
            'J' => 0,
            x => x as usize,
        }
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.typ().cmp(&other.typ());
        if ord != Ordering::Equal {
            return ord;
        }

        for (&s, &o) in self.0.cards.iter().zip(&other.0.cards) {
            let ord = Self::card_value(s).cmp(&Self::card_value(o));
            if ord != Ordering::Equal {
                return ord;
            }
        }

        panic!("There should not be any equal card sets!");
    }
}

fn main() {
    let input: String = lib::read_input!();

    let mut hands: Vec<_> = input.split('\n').map(Hand::parse).collect();
    hands.sort();

    let p1: usize = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bet * (idx + 1))
        .sum();

    p1!(p1);

    // -----------------------------------------------------------------------

    let mut hands: Vec<_> = input.split('\n').map(HandWithJoker::parse).collect();
    hands.sort();

    let p2: usize = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.0.bet * (idx + 1))
        .sum();

    p2!(p2);
}
