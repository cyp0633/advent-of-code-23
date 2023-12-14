advent_of_code::solution!(8);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut steps: u32 = 0;
    let mut curr_pos = "AAA";
    let lines = input.lines().collect::<Vec<_>>();
    let seq = lines[0];
    let mut direction = HashMap::<&str, (&str, &str)>::new();
    for line in lines[2..].iter() {
        // AAA = (BBB,CCC)
        let mut line = line.split(" = ");
        let key = line.next().unwrap();
        let mut line = line.next().unwrap().split(", ");
        let left = line.next().unwrap();
        let right = line.next().unwrap();
        direction.insert(key, (&left[1..], &right[0..3]));
    }
    while curr_pos != "ZZZ" {
        let index: usize = steps as usize % seq.len();
        curr_pos = match seq.chars().nth(index).unwrap() {
            'R' => direction.get(curr_pos).unwrap().1,
            'L' => direction.get(curr_pos).unwrap().0,
            _ => curr_pos,
        };
        steps += 1;
    }
    Some(steps)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
