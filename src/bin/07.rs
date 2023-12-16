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
    for i in 1..=hands.len() {
        winnings += hands[i - 1].bid * i as u32;
    }
    Some(winnings)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card2 {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
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

#[derive(Debug)]
struct Hand2 {
    hand: Vec<Card2>,
    bid: u32,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

impl Eq for Hand2 {}

fn merge_j(dict: &mut HashMap<&Card2, i32>) {
    // get the number of j and card with largest count
    let mut j_count = 0;
    let mut max_count = 0;
    let mut max_card = None;
    let dict_clone = dict.clone();
    for (card, count) in dict_clone.iter() {
        if card == &&Card2::J {
            j_count = *count;
        } else if *count > max_count {
            max_count = *count;
            max_card = Some(card);
        }
    }
    dict.remove(&Card2::J);
    if let Some(max_card) = max_card {
        *dict.entry(max_card).or_insert(0) += j_count;
    } else {
        dict.insert(&Card2::J, j_count);
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_dict = self.hand.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        let mut other_dict = other.hand.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        merge_j(&mut self_dict);
        merge_j(&mut other_dict);
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand2> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => Card2::A,
                    'K' => Card2::K,
                    'Q' => Card2::Q,
                    'J' => Card2::J,
                    'T' => Card2::T,
                    '9' => Card2::Nine,
                    '8' => Card2::Eight,
                    '7' => Card2::Seven,
                    '6' => Card2::Six,
                    '5' => Card2::Five,
                    '4' => Card2::Four,
                    '3' => Card2::Three,
                    '2' => Card2::Two,
                    _ => panic!("Invalid card"),
                })
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap().parse().unwrap();
            Hand2 { hand, bid }
        })
        .collect();
    hands.sort();

    let mut winnings: u32 = 0;
    for i in 1..=hands.len() {
        winnings += hands[i - 1].bid * i as u32;
        println!("#{}: {:?}", i, hands[i - 1]);
    }
    Some(winnings)
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
        assert_eq!(result, Some(5905));
    }
}
