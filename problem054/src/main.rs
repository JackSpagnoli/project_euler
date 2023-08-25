#![feature(iter_next_chunk)]

use std::fs;

use hand_rank::value_hand;

fn main() {
    println!("{}", player_1_wins("hands.txt"))
}

fn player_1_wins(file: &str) -> usize {
    fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .map(|line| line.split(' '))
        .map(|mut cards| {
            (
                cards.next_chunk::<5>().unwrap(),
                cards.next_chunk::<5>().unwrap(),
            )
        })
        .map(|(player_1_cards_strs, player_2_cards_strs)| {
            (
                parse_hand(player_1_cards_strs),
                parse_hand(player_2_cards_strs),
            )
        })
        .map(|(player_1_hand, player_2_hand)| {
            (value_hand(player_1_hand), value_hand(player_2_hand))
        })
        .fold(
            0usize,
            |player_1_wins, (player_1_hand_value, player_2_hand_value)| {
                if player_1_hand_value.cmp(&player_2_hand_value).is_ge() {
                    return player_1_wins + 1;
                }
                player_1_wins
            },
        )
}

#[derive(PartialEq, Debug)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

mod card;
mod hand_rank;

type Hand = [card::Card; 5];

fn parse_card(card_str: &str) -> card::Card {
    let mut chars = card_str.chars();
    let value: usize = match chars.next() {
        Some('A') => 14,
        Some('K') => 13,
        Some('Q') => 12,
        Some('J') => 11,
        Some('T') => 10,
        Some(val) => val.to_digit(10).unwrap() as usize,
        None => panic!("jkalJSDKLSJALIKd"),
    };
    let suit: Suit = match chars.next() {
        Some('H') => Suit::Heart,
        Some('D') => Suit::Diamond,
        Some('C') => Suit::Club,
        Some('S') => Suit::Spade,
        _ => panic!("JDKLASJDKLSJKLSAJDKS"),
    };

    card::Card { suit, value }
}

fn parse_hand(hand_str: [&str; 5]) -> Hand {
    let mut cards = hand_str.map(parse_card);
    cards.sort();
    cards
}

#[cfg(test)]
mod tests;
