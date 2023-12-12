use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    let times: Vec<u32> = lines[0]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut prod = 1;

    for (t, d) in times.iter().zip(distances.iter()) {
        let t = *t as f64;
        let d = *d as f64;
        let delta = ((t * t) - (4.0 * d)).sqrt();
        let x1 = (t + delta) / 2.0;
        let x2 = (t - delta) / 2.0;
        let count = if x1 == x1.floor() {
            x1 as u32 - 1
        } else {
            x1.floor() as u32
        } - if x2 == x2.ceil() {
            x2 as u32 + 1
        } else {
            x2.ceil() as u32
        } + 1;
        prod *= count;
    }
    Some(prod)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
