use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use regex::Regex;

/// Day 7, Part 1 -- https://adventofcode.com/2023/day/7
///
/// You've made it to Desert Island via airship! It's going to
/// take a while to ride your camel all the way from where the
/// airship dropped you and your guide-elf off, so she teaches
/// you how to playa Camel Cards, which is suspiciously similar
/// to poker but easier to play from a camel's back.
///
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
///
/// Your input is a set of "hands", consisting of the cards you
/// hold (your "hand") plus the amount of money you're betting on
/// it (your "bid"). Hands are ranked similarly to poker.
/// From strongest to weakest, they are:
///
/// Five-of-a-kind  (5)
/// four-of-a-kind  (4)
/// full-house      (3+2)
/// three-of-a-kind (4)
/// two-pair        (2+2)
/// one-pair        (2)
/// high card       (1)
///
/// If both hands ever match hand type, they're compared via
/// a tie-breaker comparing the first card in hand by strength (ace,
/// king, queen, ten, ..., three, two) one card at a time from
/// left to right until one hand beats the other.
///
/// You calculate your winnings by ordering all of your hands from
/// strongest to weakest and multiplying the "bid" of each hand
/// by its final rank. What are your total winnings?
pub fn find_camel_poker_winnings(hand_inputs: &Vec<&str>) -> u32 {
    let mut hands = hand_inputs.iter().map(|s| Hand::from_str(s)).collect_vec();

    hands.sort_by(|a, b| a.compare(b, false));

    return hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        return acc + (hand.bid * (u32::try_from(i).unwrap() + 1));
    });
}

/// Day 7, Part 2
///
/// Good news, you solved that problem! Bad news, you solved that
/// problem incorrectly! It turns out that "J" doesn't actually
/// mean "Jack", it means "Joker" which is to be treated as wild.
/// Additionally, given that a Joker's value is actually 0 and not
/// 11, you need to adjust its value when using it in a secondary
/// compare to reflect being worth less than every other card in
/// the game.
///
/// Given these adjustments, what are your new total winnings?
pub fn hand_winnings_with_jokers(hand_inputs: &Vec<&str>) -> u32 {
    let mut hands = hand_inputs
        .iter()
        .map(|s| Hand::from_str_wild(s))
        .collect_vec();

    hands.sort_by(|a, b| a.compare(b, true));

    return hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        return acc + (hand.bid * (u32::try_from(i).unwrap() + 1));
    });
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum HandType {
    HighCard = 0,
    TwoOfAKind,   // 1
    TwoPair,      // 2
    ThreeOfAKind, // etc.
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<String>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    pub fn from_str(s: &str) -> Hand {
        let parts = s.split(" ").collect_vec();
        Hand {
            cards: parts[0]
                .split("")
                .map(|s| s.to_owned())
                .filter(|s| s != "")
                .collect_vec(),
            hand_type: Hand::parse_hand_type(parts[0]),
            bid: parts[1].parse().unwrap(),
        }
    }

    pub fn from_str_wild(s: &str) -> Hand {
        let parts = s.split(" ").collect_vec();
        let re = Regex::new(r"(J)").unwrap();
        let cards_jacks_stripped = re.replace_all(parts[0], "");
        let mut default_hand = Hand {
            cards: parts[0]
                .split("")
                .map(|s| s.to_owned())
                .filter(|s| s != "")
                .collect_vec(),
            hand_type: Hand::parse_hand_type(&cards_jacks_stripped),
            bid: parts[1].parse().unwrap(),
        };
        for _i in 0..parts[0].split("").filter(|s| s == &"J").count() {
            default_hand.upgrade()
        }

        return default_hand;
    }

    pub fn parse_hand_type(hand: &str) -> HandType {
        let mut letter_map: HashMap<char, i32> = HashMap::new();
        for ch in hand.chars().collect::<Vec<char>>() {
            *letter_map.entry(ch).or_insert(0) += 1;
        }
        let counts = letter_map.values().collect_vec();

        if counts.contains(&&5) {
            return HandType::FiveOfAKind;
        } else if counts.contains(&&4) {
            return HandType::FourOfAKind;
        } else if counts.contains(&&3) && counts.contains(&&2) {
            return HandType::FullHouse;
        } else if counts.contains(&&3) {
            return HandType::ThreeOfAKind;
        } else if counts.iter().filter(|n| n == &&&2).count() == 2 {
            return HandType::TwoPair;
        } else if counts.contains(&&2) {
            return HandType::TwoOfAKind;
        }
        return HandType::HighCard;
    }

    #[rustfmt::skip]
    pub fn upgrade(&mut self) {
        match self.hand_type {
            HandType::HighCard =>     self.hand_type = HandType::TwoOfAKind,
            HandType::TwoOfAKind =>   self.hand_type = HandType::ThreeOfAKind,
            HandType::TwoPair =>      self.hand_type = HandType::FullHouse,
            HandType::ThreeOfAKind => self.hand_type = HandType::FourOfAKind,
            HandType::FullHouse =>    panic!("Impossible scenario for hand: {:?}", self),
            HandType::FourOfAKind =>  self.hand_type = HandType::FiveOfAKind,
            HandType::FiveOfAKind =>  {} // do nothing, already at best type
        }
    }

    #[rustfmt::skip]
    pub fn compare(&self, other: &Hand, wild: bool) -> Ordering {
        if self.hand_type != other.hand_type {
            if self.hand_type > other.hand_type {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }

        for i in 0..self.cards.len() {
            let v_self = if wild { wild_value(&self.cards[i]) } else { card_value(&self.cards[i]) };
            let v_other = if wild { wild_value(&other.cards[i]) } else { card_value(&other.cards[i]) };
            if v_self != v_other {
                if v_self > v_other {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
        panic!("Unexpected equality found in comparison {:?} {:?}", self.cards, other.cards);
    }
}

// Helpers for comparing positions
fn card_value(s: &str) -> usize {
    "23456789TJQKA".find(s).unwrap()
}

fn wild_value(s: &str) -> usize {
    "J23456789TQKA".find(s).unwrap()
}
