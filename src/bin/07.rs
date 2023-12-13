use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Hand {
    hand: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // how many of each card
        let self_dict = self.hand.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        let other_dict = other.hand.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        // Weight: 5 -> 30, 4 -> 20, 3 -> 10, 2 -> 5, 1 -> 1
        let self_weight = self_dict
            .iter()
            .map(|(_, count)| match count {
                5 => 30,
                4 => 20,
                3 => 10,
                2 => 5,
                1 => 1,
                _ => panic!("Invalid card count"),
            })
            .sum::<u32>();
        let other_weight = other_dict
            .iter()
            .map(|(_, count)| match count {
                5 => 30,
                4 => 20,
                3 => 10,
                2 => 5,
                1 => 1,
                _ => panic!("Invalid card count"),
            })
            .sum::<u32>();
        if self_weight > other_weight {
            return Ordering::Greater;
        } else if self_weight < other_weight {
            return Ordering::Less;
        } else {
            // compare `Card`s in descending order
            for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
                if self_card > other_card {
                    return Ordering::Greater;
                } else if self_card < other_card {
                    return Ordering::Less;
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

impl Eq for Hand {}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Invalid card"),
                })
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap().parse().unwrap();
            Hand { hand, bid }
        })
        .collect();
    hands.sort();

    let mut winnings: u32 = 0;
    for i in 1..=5 {
        winnings += hands[i - 1].bid * i as u32;
    }
    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
