use std::cmp::Ordering;

use super::{
    card::Card,
    hand_rank::{value_hand, HandRank},
    parse_card, parse_hand, player_1_wins,
    suit::Suit,
    Hand,
};

#[test]
fn test_parse_card() {
    let cases = [
        (
            "5H",
            Card {
                suit: Suit::Heart,
                value: 5,
            },
        ),
        (
            "8C",
            Card {
                suit: Suit::Club,
                value: 8,
            },
        ),
        (
            "AS",
            Card {
                suit: Suit::Spade,
                value: 14,
            },
        ),
        (
            "KD",
            Card {
                suit: Suit::Diamond,
                value: 13,
            },
        ),
        (
            "QH",
            Card {
                suit: Suit::Heart,
                value: 12,
            },
        ),
        (
            "JS",
            Card {
                suit: Suit::Spade,
                value: 11,
            },
        ),
        (
            "TD",
            Card {
                suit: Suit::Diamond,
                value: 10,
            },
        ),
    ];
    cases
        .iter()
        .for_each(|(str, card)| assert_eq!(&parse_card(str), card))
}

#[test]
fn test_parse_hand() {
    let str = ["5H", "KD", "5C", "6S", "7S"];

    let hand: Hand = [
        Card {
            suit: Suit::Heart,
            value: 5,
        },
        Card {
            suit: Suit::Club,
            value: 5,
        },
        Card {
            suit: Suit::Spade,
            value: 6,
        },
        Card {
            suit: Suit::Spade,
            value: 7,
        },
        Card {
            suit: Suit::Diamond,
            value: 13,
        },
    ];

    assert_eq!(parse_hand(str), hand)
}

#[test]
fn test_value_hand() {
    let cases = [
        (
            ["5H", "5C", "6S", "7S", "KD"],
            HandRank::Pair(5, [13, 7, 6]),
        ),
        (
            ["2C", "3S", "8S", "8D", "TD"],
            HandRank::Pair(8, [10, 3, 2]),
        ),
        (
            ["2C", "5C", "7D", "8S", "QH"],
            HandRank::HighCard([12, 8, 7, 5, 2]),
        ),
        (
            ["2D", "9C", "AS", "AH", "AC"],
            HandRank::ThreeOfAKind(14, [9, 2]),
        ),
        (
            ["3D", "6D", "7D", "TD", "QD"],
            HandRank::Flush([12, 10, 7, 6, 3]),
        ),
        (["2H", "2D", "4C", "4D", "4S"], HandRank::FullHouse(4, 2)),
        (["3C", "3D", "3S", "9S", "9D"], HandRank::FullHouse(3, 9)),
        (["AS", "QS", "KS", "JS", "TS"], HandRank::RoyalFlush),
    ];

    cases
        .iter()
        .for_each(|(str, rank)| assert_eq!(value_hand(parse_hand(*str)), *rank))
}

#[test]
fn test_hand_rank_ordering() {
    let cases = [
        (
            HandRank::Pair(5, [13, 7, 6]),
            HandRank::Pair(8, [10, 3, 2]),
            Ordering::Less,
        ),
        (
            HandRank::HighCard([14, 11, 9, 8, 5]),
            HandRank::HighCard([12, 8, 7, 5, 2]),
            Ordering::Greater,
        ),
        (
            HandRank::ThreeOfAKind(14, [9, 2]),
            HandRank::Flush([14, 10, 7, 6, 3]),
            Ordering::Less,
        ),
        (
            HandRank::Pair(12, [9, 6, 4]),
            HandRank::Pair(12, [7, 6, 3]),
            Ordering::Greater,
        ),
        (
            HandRank::FullHouse(4, 2),
            HandRank::FullHouse(3, 9),
            Ordering::Greater,
        ),
    ];

    cases
        .iter()
        .for_each(|(rank_1, rank_2, result)| assert_eq!(rank_1.cmp(rank_2), *result))
}

#[test]
fn test_file_evaluation() {
    let file_name = "resources/problem054/hands_test.txt";
    assert_eq!(player_1_wins(file_name), 3)
}
