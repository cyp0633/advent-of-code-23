advent_of_code::solution!(9);
use itertools::Itertools;
use polyfit_rs::polyfit_rs;

fn diff(nums: &Vec<i32>) -> u32 {
    let mut degree: u32 = 0;
    if nums.iter().all(|&x| x == nums[0]) {
        // special case
        return 0;
    }
    let mut diff = nums.clone();
    loop {
        // 0 3 6 9 12 15
        // -> 3 3 3 3 3
        // -> 0 0 0 0
        // then, times = 1; this is y=ax+b
        diff = diff
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        degree += 1;
        // if all diff is same, return times
        if diff.iter().all(|&x| x == diff[0]) {
            return degree;
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        let degree = diff(&nums);
        let x_values = (0..nums.len()).map(|x| x as f64).collect_vec();
        let y_values = nums.iter().map(|&x| x as f64).collect_vec();
        let coefficients = polyfit_rs::polyfit(&x_values, &y_values, degree as usize).unwrap();
        let mut next = 0.0;
        // next = f(len(nums))
        for i in 0..degree as usize + 1 {
            next += coefficients[i] * (nums.len() as f64).powi(i as i32);
        }
        println!("{}", next.round() as i64);
        sum += next.round() as i64;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        let degree = diff(&nums);
        let x_values = (0..nums.len()).map(|x| x as f64).collect_vec();
        let y_values = nums.iter().map(|&x| x as f64).collect_vec();
        let coefficients = polyfit_rs::polyfit(&x_values, &y_values, degree as usize).unwrap();
        let mut next = 0.0;
        // next = f(-1)
        for i in 0..degree as usize + 1 {
            next += coefficients[i] * (-1 as f64).powi(i as i32);
        }
        println!("{}", next.round() as i64);
        sum += next.round() as i64;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
