use std::{collections::HashMap, marker::PhantomData, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.parse::<Hand<StandardOrdering>>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid())
        .sum()
}

fn part2(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.parse::<Hand<JokerOrdering>>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid())
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card(char);

trait CardOrdering {
    fn order(card: &Card) -> i64;

    fn card_mapping(cards: Vec<Card>) -> HashMap<Card, i64>;
}

#[derive(PartialEq, Eq, Debug)]
struct StandardOrdering;

#[derive(PartialEq, Eq, Debug)]
struct JokerOrdering;

impl CardOrdering for StandardOrdering {
    fn order(card: &Card) -> i64 {
        match card.0 {
            i if i.is_numeric() => i.to_digit(10).unwrap() as i64,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            other => panic!("unexpected card: {}", other),
        }
    }

    fn card_mapping(cards: Vec<Card>) -> HashMap<Card, i64> {
        cards.into_iter().fold(HashMap::new(), |mut accum, c| {
            if let Some(existing) = accum.remove(&c) {
                accum.insert(c, existing + 1);
            } else {
                accum.insert(c, 1);
            }
            accum
        })
    }
}

impl CardOrdering for JokerOrdering {
    fn order(card: &Card) -> i64 {
        match card.0 {
            i if i.is_numeric() => i.to_digit(10).unwrap() as i64,
            'T' => 10,
            'J' => -1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            other => panic!("unexpected card: {}", other),
        }
    }

    fn card_mapping(cards: Vec<Card>) -> HashMap<Card, i64> {
        let mut card_counts = cards.into_iter().fold(HashMap::new(), |mut accum, c| {
            if let Some(existing) = accum.remove(&c) {
                accum.insert(c, existing + 1);
            } else {
                accum.insert(c, 1);
            }
            accum
        });

        // swap the highest counted card for Jokers
        if let Some(joker_count) = card_counts.remove(&Card('J')) {
            if let Some((_, count)) = card_counts
                .iter_mut()
                .sorted_by(|(_, v1), (_, v2)| Ord::cmp(v2, v1))
                .next()
            {
                *count += joker_count;
            } else {
                card_counts.insert(Card('J'), 5);
            }
        }

        card_counts
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Hand<T>
where
    T: CardOrdering,
{
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind(Vec<Card>, i64, PhantomData<T>),

    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind(Vec<Card>, i64),

    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse(Vec<Card>, i64),
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind(Vec<Card>, i64),
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair(Vec<Card>, i64),
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair(Vec<Card>, i64),
    // High card, where all cards' labels are distinct: 23456
    HighCard(Vec<Card>, i64),
}

impl<T: CardOrdering> Hand<T> {
    fn cards(&self) -> &Vec<Card> {
        match self {
            Hand::FiveOfAKind(cards, _, _) => cards,
            Hand::FourOfAKind(cards, _) => cards,
            Hand::FullHouse(cards, _) => cards,
            Hand::ThreeOfAKind(cards, _) => cards,
            Hand::TwoPair(cards, _) => cards,
            Hand::OnePair(cards, _) => cards,
            Hand::HighCard(cards, _) => cards,
        }
    }

    fn bid(&self) -> i64 {
        match self {
            Hand::FiveOfAKind(_, bid, _) => *bid,
            Hand::FourOfAKind(_, bid) => *bid,
            Hand::FullHouse(_, bid) => *bid,
            Hand::ThreeOfAKind(_, bid) => *bid,
            Hand::TwoPair(_, bid) => *bid,
            Hand::OnePair(_, bid) => *bid,
            Hand::HighCard(_, bid) => *bid,
        }
    }

    fn order(&self) -> usize {
        match self {
            Hand::FiveOfAKind(_, _, _) => 7,
            Hand::FourOfAKind(_, _) => 6,
            Hand::FullHouse(_, _) => 5,
            Hand::ThreeOfAKind(_, _) => 4,
            Hand::TwoPair(_, _) => 3,
            Hand::OnePair(_, _) => 2,
            Hand::HighCard(_, _) => 1,
        }
    }
}

impl<T: CardOrdering + std::cmp::PartialEq> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.order().cmp(&other.order()) {
            std::cmp::Ordering::Equal => {
                let self_cards = self.cards();
                let other_cards = other.cards();
                (0..5)
                    .into_iter()
                    .map(|i| {
                        let self_order = T::order(&self_cards[i]);
                        let other_order = T::order(&other_cards[i]);
                        self_order.cmp(&other_order)
                    })
                    .filter(|order| !order.is_eq())
                    .next()
            }
            other => Some(other),
        }
    }
}

impl<T> Ord for Hand<T>
where
    T: CardOrdering,
    T: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: CardOrdering> FromStr for Hand<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let cards: Vec<_> = parts
            .next()
            .expect("cards")
            .chars()
            .map(|c| Card(c))
            .collect();
        let bid = parts.next().expect("bid").parse::<i64>().expect("number");

        let card_set: HashMap<Card, i64> = T::card_mapping(cards.clone());

        match card_set.len() {
            1 => Ok(Hand::FiveOfAKind(cards, bid, PhantomData::<T>)),
            2 => {
                let first_count = *card_set.values().next().unwrap();
                match first_count {
                    1 => Ok(Hand::FourOfAKind(cards, bid)),
                    2 => Ok(Hand::FullHouse(cards, bid)),
                    3 => Ok(Hand::FullHouse(cards, bid)),
                    4 => Ok(Hand::FourOfAKind(cards, bid)),
                    _ => unreachable!("impossible hand"),
                }
            }
            3 => {
                // one of
                // Three of a Kind
                // Two Pair
                let first_count = *card_set.values().next().unwrap();
                match first_count {
                    3 => Ok(Hand::ThreeOfAKind(cards, bid)),
                    2 => Ok(Hand::TwoPair(cards, bid)),
                    1 if card_set.values().any(|v| *v == 2) => Ok(Hand::TwoPair(cards, bid)),
                    _ => Ok(Hand::ThreeOfAKind(cards, bid)),
                }
            }
            5 => Ok(Hand::HighCard(cards, bid)),
            _ => Ok(Hand::OnePair(cards, bid)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(6440, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(5905, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_order_n() {
        let mut sorted = vec![
            Card('1'),
            Card('2'),
            Card('3'),
            Card('4'),
            Card('5'),
            Card('6'),
            Card('7'),
            Card('8'),
            Card('9'),
            Card('T'),
            Card('J'),
            Card('Q'),
            Card('K'),
            Card('A'),
        ];
        let original = sorted.clone();

        sorted.sort_by(|a, b| {
            let left = StandardOrdering::order(&a);
            let right = StandardOrdering::order(&b);
            left.cmp(&right)
        });

        assert_eq!(sorted, original);
    }

    #[test]
    fn test_order_example() {
        let mut parsed: Vec<_> = include_str!("input.test.txt")
            .lines()
            .map(|l| l.parse::<Hand<StandardOrdering>>().unwrap())
            .collect();

        parsed.sort();

        assert_eq!(
            vec![
                "32T3K 765".parse::<Hand<StandardOrdering>>().unwrap(),
                "KTJJT 220".parse::<Hand<StandardOrdering>>().unwrap(),
                "KK677 28".parse::<Hand<StandardOrdering>>().unwrap(),
                "T55J5 684".parse::<Hand<StandardOrdering>>().unwrap(),
                "QQQJA 483".parse::<Hand<StandardOrdering>>().unwrap(),
            ],
            parsed
        );
    }

    #[test]
    fn test_joker_ordering_hands() {
        assert_eq!(
            Hand::FourOfAKind(
                vec![Card('T'), Card('5'), Card('5'), Card('J'), Card('5')],
                684
            ),
            "T55J5 684".parse::<Hand<JokerOrdering>>().unwrap()
        );
    }

    #[test]
    fn test_joker_order_example() {
        let mut parsed: Vec<_> = include_str!("input.test.txt")
            .lines()
            .map(|l| l.parse::<Hand<JokerOrdering>>().unwrap())
            .collect();

        parsed.sort();

        println!("cards");
        for c in &parsed {
            println!("{:?}", c);
        }

        assert_eq!(
            vec![
                "32T3K 765".parse::<Hand<JokerOrdering>>().unwrap(),
                "KK677 28".parse::<Hand<JokerOrdering>>().unwrap(),
                "T55J5 684".parse::<Hand<JokerOrdering>>().unwrap(),
                "QQQJA 483".parse::<Hand<JokerOrdering>>().unwrap(),
                "KTJJT 220".parse::<Hand<JokerOrdering>>().unwrap(),
            ],
            parsed
        );
    }

    #[test]
    fn test_regression() {
        assert_eq!(250120186, part1(include_str!("input.txt")));
    }
}
