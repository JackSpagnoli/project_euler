use std::cmp::Ordering;

use super::Hand;

#[derive(PartialEq, Debug, Eq)]
pub enum HandRank {
    HighCard([usize; 5]),
    Pair(usize, [usize; 3]),
    TwoPair(usize, usize, [usize; 1]),
    ThreeOfAKind(usize, [usize; 2]),
    Straight(usize),
    Flush([usize; 5]),
    FullHouse(usize, usize),
    FourOfAKind(usize, [usize; 1]),
    StraightFlush(usize),
    RoyalFlush,
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank_order = rank_order(self);
        let other_rank_order = rank_order(other);
        match self_rank_order.cmp(&other_rank_order) {
            Ordering::Equal => match (self, other) {
                (HandRank::HighCard(self_hand), HandRank::HighCard(other_hand))
                | (HandRank::Flush(self_hand), HandRank::Flush(other_hand)) => {
                    self_hand.cmp(other_hand)
                }
                (HandRank::Straight(self_start_val), HandRank::Straight(other_start_val))
                | (
                    HandRank::StraightFlush(self_start_val),
                    HandRank::StraightFlush(other_start_val),
                ) => self_start_val.cmp(other_start_val),
                (
                    HandRank::Pair(self_pair_val, self_hand),
                    HandRank::Pair(other_pair_val, other_hand),
                ) => match self_pair_val.cmp(other_pair_val) {
                    Ordering::Equal => self_hand.cmp(other_hand),
                    ordering => ordering,
                },
                (
                    HandRank::FourOfAKind(self_pair_val, self_hand),
                    HandRank::FourOfAKind(other_pair_val, other_hand),
                ) => match self_pair_val.cmp(other_pair_val) {
                    Ordering::Equal => self_hand.cmp(other_hand),
                    ordering => ordering,
                },
                (
                    HandRank::ThreeOfAKind(self_pair_val, self_hand),
                    HandRank::ThreeOfAKind(other_pair_val, other_hand),
                ) => match self_pair_val.cmp(other_pair_val) {
                    Ordering::Equal => self_hand.cmp(other_hand),
                    ordering => ordering,
                },
                (
                    HandRank::FullHouse(self_triple_val, self_pair_val),
                    HandRank::FullHouse(other_triple_val, other_pair_val),
                ) => match self_triple_val.cmp(other_triple_val) {
                    Ordering::Equal => self_pair_val.cmp(other_pair_val),
                    ordering => ordering,
                },
                (
                    HandRank::TwoPair(self_first_pair_val, self_second_pair_val, self_hand),
                    HandRank::TwoPair(other_first_pair_val, other_second_pair_val, other_hand),
                ) => match self_first_pair_val.cmp(other_first_pair_val) {
                    Ordering::Equal => match self_second_pair_val.cmp(other_second_pair_val) {
                        Ordering::Equal => self_hand.cmp(other_hand),
                        ordering => ordering,
                    },
                    ordering => ordering,
                },
                _ => panic!("askljslk"),
            },
            order => order,
        }
    }
}

fn rank_order(hand_rank: &HandRank) -> usize {
    match hand_rank {
        HandRank::HighCard(_) => 1,
        HandRank::Pair(..) => 2,
        HandRank::TwoPair(..) => 3,
        HandRank::ThreeOfAKind(..) => 4,
        HandRank::Straight(_) => 5,
        HandRank::Flush(_) => 6,
        HandRank::FullHouse(..) => 7,
        HandRank::FourOfAKind(..) => 8,
        HandRank::StraightFlush(_) => 9,
        HandRank::RoyalFlush => 10,
    }
}

pub fn value_hand(hand: Hand) -> HandRank {
    let first_card_suit = &hand[0].suit;
    let is_flush = hand[1..].iter().all(|hand| hand.suit == *first_card_suit);
    let is_straight = hand
        .windows(2)
        .all(|cards| cards[0].value == cards[1].value - 1);

    let match_pattern = calc_match_pattern(&hand);

    // println!("match_pattern: {match_pattern:?}");

    let values: [usize; 5] = hand.map(|card| card.value);
    let reversed_values = values
        .iter()
        .rev()
        .copied()
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    match (is_flush, is_straight, values[0]) {
        (true, true, 10) => return HandRank::RoyalFlush,
        (true, true, _) => return HandRank::StraightFlush(values[0]),
        (true, false, _) => return HandRank::Flush(reversed_values),
        (false, true, _) => return HandRank::Straight(values[0]),
        _ => {}
    }

    match match_pattern[..] {
        [4, 1] => return HandRank::FourOfAKind(values[0], [values[4]]),
        [1, 4] => return HandRank::FourOfAKind(values[1], [values[0]]),
        [3, 2] => return HandRank::FullHouse(values[0], values[3]),
        [2, 3] => return HandRank::FullHouse(values[2], values[0]),
        [3, 1, 1] => return HandRank::ThreeOfAKind(values[0], [values[4], values[3]]),
        [1, 3, 1] => return HandRank::ThreeOfAKind(values[1], [values[4], values[0]]),
        [1, 1, 3] => return HandRank::ThreeOfAKind(values[3], [values[1], values[0]]),
        [1, 2, 2] => return HandRank::TwoPair(values[3], values[1], [values[0]]),
        [2, 1, 2] => return HandRank::TwoPair(values[3], values[0], [values[2]]),
        [2, 2, 1] => return HandRank::TwoPair(values[2], values[0], [values[4]]),
        _ => {}
    }

    if !match_pattern.contains(&2) {
        return HandRank::HighCard(reversed_values);
    }

    let index_of_pair = match_pattern.iter().position(|value| value == &2).unwrap();
    let lower_cards = &values[0..index_of_pair];
    let higher_cards = &values[index_of_pair + 2..5];
    let non_pair_cards: [usize; 3] = [lower_cards, higher_cards]
        .concat()
        .iter()
        .copied()
        .rev()
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    HandRank::Pair(values[index_of_pair], non_pair_cards)
}

fn calc_match_pattern(hand: &Hand) -> Vec<usize> {
    let (mut match_patterns, final_pattern) = hand.windows(2).fold(
        (vec![], 1usize),
        |(mut match_pattern, mut current_matches), cards| {
            if cards[0].value == cards[1].value {
                current_matches += 1;
            } else {
                match_pattern.push(current_matches);
                current_matches = 1;
            }
            (match_pattern, current_matches)
        },
    );

    match_patterns.push(final_pattern);
    match_patterns
}
