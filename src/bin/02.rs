advent_of_code::solution!(2);
use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let mut id_sum: u32 = 0;
    'game: for (index, game) in input.lines().enumerate() {
        // split "Game X: yyyyyyyyy"
        let content = game.split(": ").collect::<Vec<&str>>()[1];
        // split "X blue, Y red; XX green, YY yellow"
        let gotchas = content.split("; ").collect::<Vec<&str>>();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        // Per gotcha: maximum 12 red, 13 green, 14 blue
        for gotcha in gotchas {
            let colors = gotcha.split(", ").collect::<Vec<&str>>();
            for color in colors {
                // color: "X blue/green/red"
                let color = color.split(" ").collect::<Vec<&str>>();
                let count = color[0].parse::<u32>().unwrap();
                match color[1] {
                    "blue" => blue = count,
                    "green" => green = count,
                    "red" => red = count,
                    _ => panic!("Unknown color: {}", color[1]),
                }
            }
            if red > 12 || green > 13 || blue > 14 {
                // this game is invalid, skip to next game
                continue 'game;
            }
        }
        id_sum += (index + 1) as u32;
    }
    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut id_sum: u32 = 0;
    for game in input.lines() {
        // split "Game X: yyyyyyyyy"
        let content = game.split(": ").collect::<Vec<&str>>()[1];
        // split "X blue, Y red; XX green, YY yellow"
        let gotchas = content.split("; ").collect::<Vec<&str>>();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        // Get the maximum of each color
        for gotcha in gotchas {
            let colors = gotcha.split(", ").collect::<Vec<&str>>();
            for color in colors {
                // color: "X blue/green/red"
                let color = color.split(" ").collect::<Vec<&str>>();
                let count = color[0].parse::<u32>().unwrap();
                match color[1] {
                    "blue" => blue = cmp::max(blue, count),
                    "green" => green = cmp::max(green, count),
                    "red" => red = cmp::max(red, count),
                    _ => panic!("Unknown color: {}", color[1]),
                }
            }
        }
        let color_pow = red * green * blue;
        id_sum += color_pow;
    }
    Some(id_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
