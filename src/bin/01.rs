advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    // separate input into lines
    let lines = input.lines();
    let mut sum = 0;
    let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for line in lines {
        let mut first_number = 0;
        let mut first_index: usize = line.len() + 1;
        let mut last_number = 0;
        let mut last_index: usize = 0;

        for i in 0..10 {
            if let Some(number_first_match) = line.find(numbers[i]) {
                if number_first_match < first_index {
                    first_number = i;
                    first_index = number_first_match;
                }
            }
            if let Some(number_last_match) = line.rfind(numbers[i]) {
                if number_last_match >= last_index { // If the only number is at [0], use this to record it
                    last_number = i;
                    last_index = number_last_match;
                }
            }
        }
        sum += first_number * 10 + last_number;
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
        // separate input into lines
        let lines = input.lines();
        let mut sum = 0;
        let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let word_numbers = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for line in lines {
            let mut first_number = 0;
            let mut first_index: usize = line.len() + 1;
            let mut last_number = 0;
            let mut last_index: usize = 0;

            for i in 0..10 {
                if let Some(word_first_match) = line.find(word_numbers[i]) {
                    if word_first_match < first_index {
                        first_number = i;
                        first_index = word_first_match;
                    }
                }
                if let Some(word_last_match) = line.rfind(word_numbers[i]) {
                    if word_last_match >= last_index {
                        last_number = i;
                        last_index = word_last_match;
                    }
                }
                if let Some(number_first_match) = line.find(numbers[i]) {
                    if number_first_match < first_index {
                        first_number = i;
                        first_index = number_first_match;
                    }
                }
                if let Some(number_last_match) = line.rfind(numbers[i]) {
                    if number_last_match >= last_index {
                        last_number = i;
                        last_index = number_last_match;
                    }
                }
            }
            sum += first_number * 10 + last_number;
        }
        Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
