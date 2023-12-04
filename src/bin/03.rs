use std::char;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // convert lines into a char matrix
    let lines = input.lines();
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().collect());
    }

    let mut curr = 0; // current number
    let mut is_part_number = false; // current number adjacent to a symbol
                                    // iterate through the matrix
    for (x, line) in matrix.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if *ch >= '0' && *ch <= '9' {
                // ch is a number
                curr = curr * 10 + (*ch as u32 - '0' as u32);
                if !is_part_number {
                    // check char in 8 directions
                    if x > 0 && is_symbol(matrix[x - 1][y]) {
                        is_part_number = true;
                    } else if y > 0 && is_symbol(matrix[x][y - 1]) {
                        is_part_number = true;
                    } else if x < matrix.len() - 1 && is_symbol(matrix[x + 1][y]) {
                        is_part_number = true;
                    } else if y < matrix[x].len() - 1 && is_symbol(matrix[x][y + 1]) {
                        is_part_number = true;
                    } else if x > 0 && y > 0 && is_symbol(matrix[x - 1][y - 1]) {
                        is_part_number = true;
                    } else if x > 0 && y < matrix[x].len() - 1 && is_symbol(matrix[x - 1][y + 1]) {
                        is_part_number = true;
                    } else if x < matrix.len() - 1 && y > 0 && is_symbol(matrix[x + 1][y - 1]) {
                        is_part_number = true;
                    } else if x < matrix.len() - 1
                        && y < matrix[x].len() - 1
                        && is_symbol(matrix[x + 1][y + 1])
                    {
                        is_part_number = true;
                    }
                }
            } else {
                // ch is a letter
                if curr != 0 {
                    if is_part_number {
                        sum += curr;
                        println!("Add {}", curr)
                    }
                    curr = 0;
                    is_part_number = false;
                }
            }
        }
        // at the end of each line (including last), add the number
        if curr != 0 {
            if is_part_number {
                sum += curr;
                println!("Add {}", curr)
            }
            curr = 0;
            is_part_number = false;
        }
    }
    Some(sum)
}

fn is_symbol(char: char) -> bool {
    (char < '0' || char > '9') && char != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
