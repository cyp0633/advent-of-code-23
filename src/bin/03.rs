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

pub fn part_two(input: &str) -> Option<u32> {
    // convert lines into a char matrix
    let lines = input.lines();
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().collect());
    }

    for (x, line) in matrix.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if *ch == '*' {
                // find numbers nearby
                let (
                    mut up_left,
                    mut up,
                    mut up_right,
                    mut left,
                    mut right,
                    mut down_left,
                    mut down,
                    mut down_right,
                ) = (false, false, false, false, false, false, false, false);
                let mut pow = 1;
                if x > 0 {
                    if y > 0 {
                        if is_digit(matrix[x - 1][y - 1]) {
                            up_left = true;
                        }
                    }
                    if is_digit(matrix[x - 1][y]) {
                        up = true;
                    }
                    if y < matrix[x].len() - 1 {
                        if is_digit(matrix[x - 1][y + 1]) {
                            up_right = true;
                        }
                    }
                }
                if y > 0 && is_digit(matrix[x][y - 1]) {
                    left = true;
                }
                if y < matrix[x].len() - 1 && is_digit(matrix[x][y + 1]) {
                    right = true;
                }
                if x < matrix.len() - 1 {
                    if y > 0 {
                        if is_digit(matrix[x + 1][y - 1]) {
                            down_left = true;
                        }
                    }
                    if is_digit(matrix[x + 1][y]) {
                        down = true;
                    }
                    if y < matrix[x].len() - 1 {
                        if is_digit(matrix[x + 1][y + 1]) {
                            down_right = true;
                        }
                    }
                }
                // get number count
                let mut count = 0;
                match (up_left, up, up_right) {
                    (false, false, true) | (true, false, false) | (false, true, false) => {
                        count += 1
                    }
                    (true, true, false) | (false, true, true) => {
                        count += 1;
                        up = false; // to prevent calculating value twice
                    }
                    (true, false, true) => count += 2,
                    (true, true, true) => {
                        count += 1;
                        up_left = false; // as above
                        up_right = false;
                    }
                    (false, false, false) => {}
                }
                if left {
                    count += 1;
                }
                if right {
                    count += 1;
                }
                match (down_left, down, down_right) {
                    (false, false, true) | (true, false, false) | (false, true, false) => {
                        count += 1
                    }
                    (true, true, false) | (false, true, true) => {
                        count += 1;
                        down = false;
                    }
                    (true, false, true) => count += 2,
                    (true, true, true) => {
                        count += 1;
                        down_left = false;
                        down_right = false;
                    }
                    (false, false, false) => {}
                }
                println!("({}, {}) has {} numbers", x, y, count);
                if count != 2 {
                    // not a "gear"
                    continue;
                }
                if up_left {
                    pow *= get_value(&matrix[x - 1], y - 1);
                }
                if up {
                    pow *= get_value(&matrix[x - 1], y);
                }
                if up_right {
                    pow *= get_value(&matrix[x - 1], y + 1);
                }
                if left {
                    pow *= get_value(&matrix[x], y - 1);
                }
                if right {
                    pow *= get_value(&matrix[x], y + 1);
                }
                if down_left {
                    pow *= get_value(&matrix[x + 1], y - 1);
                }
                if down {
                    pow *= get_value(&matrix[x + 1], y);
                }
                if down_right {
                    pow *= get_value(&matrix[x + 1], y + 1);
                }
                sum += pow;
                println!("Adding {} at ({}, {})", pow, x, y);
            }
        }
    }
    Some(sum as u32)
}

fn is_symbol(char: char) -> bool {
    (char < '0' || char > '9') && char != '.'
}

fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

// Since the value stays only on one line, we can just use one line.
fn get_value(matrix: &Vec<char>, y: usize) -> i32 {
    let mut start_y = y;
    let mut val = 0;
    while start_y > 0 && is_digit(matrix[start_y - 1]) {
        start_y -= 1;
    }
    while start_y < matrix.len() && is_digit(matrix[start_y]) {
        val = val * 10 + (matrix[start_y] as i32 - '0' as i32);
        start_y += 1;
    }
    val
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
        assert_eq!(result, Some(467835));
    }
}
