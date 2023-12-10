use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let mut win = 0;
        // process strings
        let content = line.split(": ").collect_vec()[1]; // cut "Card X: "
        let result = content.split(" | ").collect_vec();
        let winning_numbers = result[0];
        let own_numbers = result[1];
        let winning_set: HashSet<u32> = winning_numbers
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let own_numbers: Vec<u32> = own_numbers
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        for number in own_numbers.iter() {
            if winning_set.contains(number) {
                win += 1;
            }
        }
        if win > 0 {
            sum += 2_u32.pow(win - 1);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    let mut sum = 0; // cards scratched in total
    let mut cards_count = vec![1; lines.len()]; // every card has 1 copy
    let contents: Vec<&str> = lines
        .iter()
        .map(|x| x.split(": ").collect_vec()[1])
        .collect_vec();
    let results = contents
        .iter()
        .map(|x| x.split(" | ").collect_vec())
        .collect_vec();
    let winning_sets: Vec<HashSet<u32>> = results
        .iter()
        .map(|x| x[0])
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<u32>().unwrap()) // Numbers that gets point for each card
                .collect()
        })
        .collect_vec();
    let own_numbers: Vec<Vec<u32>> = results
        .iter()
        .map(|x| x[1])
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<u32>().unwrap()) // Numbers that gets point for each card
                .collect()
        })
        .collect_vec();
    for index in 0..cards_count.len() {
        println!("Has {} of card {}", cards_count[index], index);
        if cards_count[index] == 0 {
            break;
        }
        sum += cards_count[index];
        let winning_set = &winning_sets[index];
        let own_numbers = &own_numbers[index];
        let mut win = 0;
        print!("Intersect numbers: ");
        for number in own_numbers.iter() {
            if winning_set.contains(number) {
                win += 1;
                print!("{} ", number);
            }
        }
        println!("Win {} cards", win,);
        for i in 0..win {
            cards_count[index + 1 + i] += cards_count[index];
            println!(
                "Add {} copies of card {}",
                cards_count[index],
                index + 2 + i
            )
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
