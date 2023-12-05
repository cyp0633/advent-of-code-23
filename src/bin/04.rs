use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines{
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
    None
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
        assert_eq!(result, None);
    }
}
