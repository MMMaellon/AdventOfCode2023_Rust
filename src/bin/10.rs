advent_of_code::solution!(10);

fn find_branches(tiles: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut branches: Vec<(usize, usize)> = Vec::new();
    let tile = tiles[y][x];
    //check top
    if y > 0 {
        match tile {
            'S' | '|' | 'L' | 'J' => match tiles[y - 1][x] {
                '|' | '7' | 'F' => {
                    branches.push((x, y - 1));
                }
                _ => {}
            },
            _ => {}
        }
    }
    if y < tiles.len() - 1 {
        match tile {
            'S' | '|' | '7' | 'F' => match tiles[y + 1][x] {
                '|' | 'J' | 'L' => {
                    branches.push((x, y + 1));
                }
                _ => {}
            },
            _ => {}
        }
    }

    if x > 0 {
        match tile {
            'S' | '-' | 'L' | 'F' => match tiles[y][x - 1] {
                '-' | 'J' | '7' => {
                    branches.push((x - 1, y));
                }
                _ => {}
            },
            _ => {}
        }
    }

    if x < tiles[0].len() - 1 {
        match tile {
            'S' | '-' | 'J' | '7' => match tiles[y][x + 1] {
                '-' | 'L' | 'F' => {
                    branches.push((x + 1, y));
                }
                _ => {}
            },
            _ => {}
        }
    }
    return branches;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tiles: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut width: usize = 0;
    let mut height: usize = 0;
    for (y, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        width = chars.len();
        for (x, char) in chars.iter().enumerate() {
            if *char == 'S' {
                start = (x, y);
            }
        }
        tiles.push(chars);
        height = y + 1;
    }
    println!("start {:?}", start);
    println!("start {:?}", tiles[start.1][start.0]);
    let mut distances = vec![vec![0; width]; height];
    let mut greatest_distance = 0;
    let mut branches = find_branches(&tiles, start);
    while branches.len() > 0 {
        let mut new_branches = Vec::new();
        greatest_distance += 1;
        for branch in branches.iter() {
            println!("branch {:?}{}", branch , tiles[branch.1][branch.0]);
            let next_step = find_branches(&tiles, *branch);
            for (x, y) in next_step.iter(){
                if distances[*y][*x] == 0{
                    distances[*y][*x] = greatest_distance;
                    new_branches.push((*x, *y));
                }
            }
        }
        branches = new_branches;
        println!("distance: {}", greatest_distance);
        for line in distances.iter(){
            println!("{:?}", line);
        }
    }
    Some(greatest_distance - 1)
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
