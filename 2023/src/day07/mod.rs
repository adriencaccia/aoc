use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap};
use strum::{EnumIter, IntoEnumIterator};

use itertools::Itertools;

#[derive(EnumIter, Eq, PartialEq, Hash, Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

lazy_static! {
    static ref CARD_ORDER: HashMap<char, u8> =
        ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (c, i as u8))
            .collect();
    static ref CARD_ORDER_JOKER: HashMap<char, u8> =
        ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (c, i as u8))
            .collect();
    static ref HAND_TYPE_ORDER: HashMap<HandType, u8> = HandType::iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect();
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    pub fn from_line(line: &str, joker: bool) -> Self {
        let (cards_str, bid_str) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let mut card_counter: HashMap<char, u8> = HashMap::new();
        for char in cards_str.chars() {
            card_counter
                .entry(char)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
        if let (true, false, Some(j_count)) = (
            joker,
            card_counter.len() == 1,
            card_counter.get(&'J').cloned(),
        ) {
            let (max_key, _) = card_counter
                .iter()
                .filter(|(&k, _)| k != 'J')
                .max_by_key(|(_, &v)| v)
                .unwrap();
            card_counter.entry(*max_key).and_modify(|v| *v += j_count);
            card_counter.remove_entry(&'J');
        }
        let hand_type = match card_counter.len() {
            1 => HandType::FiveKind,
            2 if card_counter.values().any(|&n| n == 4) => HandType::FourKind,
            2 => HandType::FullHouse,
            3 if card_counter.values().any(|&n| n == 3) => HandType::ThreeKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Self {
            cards: cards_str.into(),
            hand_type,
            bid: bid_str.parse().unwrap(),
        }
    }

    fn cmp_with_order(&self, other: &Self, card_order: &HashMap<char, u8>) -> Ordering {
        let self_type_rank = HAND_TYPE_ORDER.get(&self.hand_type).unwrap();
        let other_type_rank = HAND_TYPE_ORDER.get(&other.hand_type).unwrap();

        let order_rank = self_type_rank.cmp(other_type_rank);
        if order_rank == Ordering::Equal {
            let self_cards_ranks = self.cards.chars().map(|c| card_order.get(&c).unwrap());
            let other_cards_ranks = other.cards.chars().map(|c| card_order.get(&c).unwrap());

            return self_cards_ranks.cmp(other_cards_ranks);
        }

        order_rank
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let lines = input.trim().lines();

    let part1 = lines
        .clone()
        .map(|line| Hand::from_line(line, false))
        .sorted_by(|a, b| Hand::cmp_with_order(a, b, &CARD_ORDER))
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + hand.bid * (rank as u32 + 1));

    let part2 = lines
        .clone()
        .map(|line| Hand::from_line(line, true))
        .sorted_by(|a, b| Hand::cmp_with_order(a, b, &CARD_ORDER_JOKER))
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + hand.bid * (rank as u32 + 1));

    (part1, part2)
}

pub fn main() -> (u32, u32) {
    let (part1, part2) = parse_input(include_str!("input.txt"));
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 6440);
        assert_eq!(part2, 5905);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 256448566);
        assert_eq!(part2, 254412181);
    }
}
