use std::collections::HashSet;
use std::thread;

advent_of_code::solution!(23);

fn parse(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|x| x.chars().collect()).collect();
}

fn find_start(tiles: &Vec<Vec<char>>) -> (usize, usize) {
    let (mut x, y) = (0, 0);
    for (i, tile) in tiles[0].iter().enumerate() {
        if *tile == '.' {
            x = i;
            break;
        }
    }
    return (x, y);
}
fn find_end(tiles: &Vec<Vec<char>>) -> (usize, usize) {
    let (mut x, y) = (0, tiles.len() - 1);
    if !tiles.is_empty() {
        for (i, tile) in tiles[tiles.len() - 1].iter().enumerate() {
            if *tile == '.' {
                x = i;
                break;
            }
        }
    }
    return (x, y);
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn longest_path_dry(
    tiles: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    target: (usize, usize),
    visited: HashSet<(usize, usize)>,
    path: Vec<(usize, usize)>,
    memo: &mut Vec<Vec<Vec<(usize, usize)>>>,
) -> Option<Vec<(usize, usize)>> {
    let mut x = x;
    let mut y = y;
    let mut prev_x = x;
    let mut prev_y = y;
    let mut new_path = path.clone();
    let mut new_visited = visited.clone();
    let mut looped = false;
    let mut prev_direction = Direction::Left;
    loop {
        // if y >= tiles.len()
        //     || x >= tiles[y].len()
        //     || tiles[y][x] == '#'
        //     || visited.contains(&(x, y))
        // {
        //     return None;
        // }

        let mut results = Vec::new();

        new_path.push((x, y));
        new_visited.insert((x, y));

        if (x, y) == target {
            return Some(path);
        }

        let mut branch_count = 0;
        let mut next_x = x;
        let mut next_y = y;
        let mut left = false;
        let mut right = false;
        let mut up = false;
        let mut down = false;
        if x > 0 && tiles[y][x - 1] != '#' && !visited.contains(&(x - 1, y)) {
            branch_count += 1;
            next_x -= 1;
            left = true;
            if !looped {
                prev_direction = Direction::Left;
            }
        }
        if x < tiles[y].len() - 1 && tiles[y][x + 1] != '#' && !visited.contains(&(x + 1, y)) {
            branch_count += 1;
            next_x += 1;
            right = true;
            if !looped {
                prev_direction = Direction::Right;
            }
        }
        if y > 0 && tiles[y - 1][x] != '#' && !visited.contains(&(x, y - 1)) {
            branch_count += 1;
            next_y -= 1;
            up = true;
            if !looped {
                prev_direction = Direction::Up;
            }
        }
        if y < tiles.len() - 1 && tiles[y + 1][x] != '#' && !visited.contains(&(x, y + 1)) {
            branch_count += 1;
            next_y += 1;
            down = true;
            if !looped {
                prev_direction = Direction::Down;
            }
        }

        if branch_count == 1 {
            if !looped {
                prev_x = x;
                prev_y = y;
                looped = true;
                x = next_x;
                y = next_y;
            } else if left && memo[0][y][x] != (0, 0){
                (x, y) = memo[0][y][x];
            } else if right && memo[1][y][x] != (0, 0){
                (x, y) = memo[1][y][x];
            } else if up && memo[2][y][x] != (0, 0){
                (x, y) = memo[2][y][x];
            } else if down && memo[3][y][x] != (0, 0){
                (x, y) = memo[3][y][x];
            } else {
                x = next_x;
                y = next_y;
            }
            continue;
        }

        if looped {
            match prev_direction{
                Direction::Left => {
                    memo[0][prev_y][prev_y] = (x, y);
                }
                Direction::Right => {
                    memo[1][prev_y][prev_y] = (x, y);
                }
                Direction::Up => {
                    memo[2][prev_y][prev_y] = (x, y);
                }
                Direction::Down => {
                    memo[3][prev_y][prev_y] = (x, y);
                }
            }
        }

        let mut max = 0;
        if left {
            results.push(longest_path_dry(
                tiles,
                (x - 1, y),
                target,
                new_visited.clone(),
                new_path.clone(),
                memo,
            ));
        }
        if right {
            results.push(longest_path_dry(
                tiles,
                (x + 1, y),
                target,
                new_visited.clone(),
                new_path.clone(),
                memo,
            ));
        }

        if up {
            results.push(longest_path_dry(
                tiles,
                (x, y - 1),
                target,
                new_visited.clone(),
                new_path.clone(),
                memo,
            ));
        }
        if down {
            results.push(longest_path_dry(
                tiles,
                (x, y + 1),
                target,
                new_visited,
                new_path,
                memo,
            ));
        }

        let mut filtered_results = results.into_iter().filter_map(|x| x);
        for result in filtered_results.clone() {
            max = max.max(result.len());
        }

        let winner = filtered_results.find(|path| path.len() == max);

        return winner;
    }
}

fn longest_path(
    tiles: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    target: (usize, usize),
    visited: HashSet<(usize, usize)>,
    path: Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    if y >= tiles.len() || x >= tiles[y].len() || tiles[y][x] == '#' || visited.contains(&(x, y)) {
        return None;
    }

    let mut results = Vec::new();
    let mut new_path = path.clone();
    let mut new_visited = visited.clone();

    new_path.push((x, y));
    new_visited.insert((x, y));

    if (x, y) == target {
        return Some(path);
    }

    match tiles[y][x] {
        '<' => {
            if x == 0 {
                return None;
            }
            return longest_path(tiles, (x - 1, y), target, new_visited, new_path);
        }
        '>' => {
            return longest_path(tiles, (x + 1, y), target, new_visited, new_path);
        }
        '^' => {
            if y == 0 {
                return None;
            }
            return longest_path(tiles, (x, y - 1), target, new_visited, new_path);
        }
        'v' => {
            return longest_path(tiles, (x, y + 1), target, new_visited, new_path);
        }
        _ => {}
    }
    let mut max = 0;
    if x > 0 {
        results.push(longest_path(
            tiles,
            (x - 1, y),
            target,
            new_visited.clone(),
            new_path.clone(),
        ));
    }
    results.push(longest_path(
        tiles,
        (x + 1, y),
        target,
        new_visited.clone(),
        new_path.clone(),
    ));

    if y > 0 {
        results.push(longest_path(
            tiles,
            (x, y - 1),
            target,
            new_visited.clone(),
            new_path.clone(),
        ));
    }
    results.push(longest_path(
        tiles,
        (x, y + 1),
        target,
        new_visited,
        new_path,
    ));

    let mut filtered_results = results.into_iter().filter_map(|x| x);
    for result in filtered_results.clone() {
        max = max.max(result.len());
    }

    let winner = filtered_results.find(|path| path.len() == max);

    return winner;
}

const STACK_SIZE: usize = 4 * 1024 * 1024;

fn part_one_thread(input: &str) -> Option<usize> {
    let tiles = parse(input);
    let start = find_start(&tiles);
    let end = find_end(&tiles);

    let longest = longest_path(&tiles, start, end, HashSet::new(), Vec::new());

    return Some(longest?.len());
}
fn part_two_thread(input: &str) -> Option<usize> {
    let tiles = parse(input);
    let start = find_start(&tiles);
    let end = find_end(&tiles);

    let mut memo = vec![vec![vec![(0,0);tiles[0].len()];tiles.len()];4];
    let longest = longest_path_dry(&tiles, start, end, HashSet::new(), Vec::new(), &mut memo);

    return Some(longest?.len());
}
pub fn part_one(input: &str) -> Option<usize> {
    let new_input = input.to_owned();
    let clojure = move || part_one_thread(&new_input);
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(clojure)
        .unwrap();
    return child.join().unwrap();
}

pub fn part_two(input: &str) -> Option<usize> {
    let new_input = input.to_owned();
    let clojure = move || part_two_thread(&new_input);
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE * 10)
        .spawn(clojure)
        .unwrap();
    return child.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
