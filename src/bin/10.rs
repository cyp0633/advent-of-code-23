advent_of_code::solution!(10);

static LEFT_CONNECT: [char; 3] = ['-', 'L', 'F'];
static RIGHT_CONNECT: [char; 3] = ['-', 'J', '7'];
static TOP_CONNECT: [char; 3] = ['|', 'F', '7'];
static BOTTOM_CONNECT: [char; 3] = ['|', 'J', 'L'];

fn dfs(map: &Vec<Vec<char>>, map_dist: &mut Vec<Vec<Option<u32>>>, x: usize, y: usize) {
    let curr = map[x as usize][y as usize];
    let curr_dist = map_dist[x as usize][y as usize];
    if curr_dist.is_none() {
        return;
    }

    if x > 0 && TOP_CONNECT.contains(&map[x - 1][y]) && BOTTOM_CONNECT.contains(&curr) {
        let new_dist = curr_dist.unwrap() + 1;
        if map_dist[x - 1][y].is_none() || map_dist[x - 1][y].unwrap() > new_dist {
            map_dist[x - 1][y] = Some(new_dist);
            dfs(map, map_dist, x - 1, y);
        }
    }
    if x < map.len() - 1 && BOTTOM_CONNECT.contains(&map[x + 1][y]) && TOP_CONNECT.contains(&curr) {
        let new_dist = curr_dist.unwrap() + 1;
        if map_dist[x + 1][y].is_none() || map_dist[x + 1][y].unwrap() > new_dist {
            map_dist[x + 1][y] = Some(new_dist);
            dfs(map, map_dist, x + 1, y);
        }
    }
    if y > 0 && LEFT_CONNECT.contains(&map[x][y - 1]) && RIGHT_CONNECT.contains(&curr) {
        let new_dist = curr_dist.unwrap() + 1;
        if map_dist[x][y - 1].is_none() || map_dist[x][y - 1].unwrap() > new_dist {
            map_dist[x][y - 1] = Some(new_dist);
            dfs(map, map_dist, x, y - 1);
        }
    }
    if y < map[x].len() - 1
        && RIGHT_CONNECT.contains(&map[x][y + 1])
        && LEFT_CONNECT.contains(&curr)
    {
        let new_dist = curr_dist.unwrap() + 1;
        if map_dist[x][y + 1].is_none() || map_dist[x][y + 1].unwrap() > new_dist {
            map_dist[x][y + 1] = Some(new_dist);
            dfs(map, map_dist, x, y + 1);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_dist = 0;
    // array of array of chars
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut map_dist = vec![vec![None; map[0].len()]; map.len()];
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            // Don't just dfs (x,y); it doesn't have a pipe
            if map[x][y] != 'S' {
                continue;
            }
            if x > 0 && TOP_CONNECT.contains(&map[x - 1][y]) {
                map_dist[x - 1][y] = Some(1);
                dfs(&map, &mut map_dist, x - 1, y);
            }
            if x < map.len() - 1 && BOTTOM_CONNECT.contains(&map[x + 1][y]) {
                map_dist[x + 1][y] = Some(1);
                dfs(&map, &mut map_dist, x + 1, y);
            }
            if y > 0 && LEFT_CONNECT.contains(&map[x][y - 1]) {
                map_dist[x][y - 1] = Some(1);
                dfs(&map, &mut map_dist, x, y - 1);
            }
            if y < map[x].len() - 1 && RIGHT_CONNECT.contains(&map[x][y + 1]) {
                map_dist[x][y + 1] = Some(1);
                dfs(&map, &mut map_dist, x, y + 1);
            }
        }
    }
    for row in map_dist {
        for col in row {
            if let Some(dist) = col {
                max_dist = max_dist.max(dist);
            }
        }
    }
    Some(max_dist)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
