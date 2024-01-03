
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
            'S' | '-' | 'J' | '7' => match tiles[y][x - 1] {
                '-' | 'L' | 'F' => {
                    branches.push((x - 1, y));
                }
                _ => {}
            },
            _ => {}
        }
    }

    if x < tiles[0].len() - 1 {
        match tile {
            'S' | '-' | 'L' | 'F' => match tiles[y][x + 1] {
                '-' | 'J' | '7' => {
                    branches.push((x + 1, y));
                }
                _ => {}
            },
            _ => {}
        }
    }
    // println!("Found {} branches", branches.len());
    return branches;
}

fn print_highlighted(tiles: &Vec<Vec<char>>, highlighted: &Vec<(usize, usize)>){
    for (y, row) in tiles.iter().enumerate(){
        let mut line : String = String::new();
        for (x, tile) in row.iter().enumerate(){
            if highlighted.contains(&(x,y)){
                line = format!("{}\x1b[93m{}\x1b[0m", line, tile);
            } else {
                line.push(*tile);
            }
        }
        println!("{}", line);
    }
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
    let mut distances = vec![vec![0; width]; height];
    let mut greatest_distance = 0;
    let mut branches = find_branches(&tiles, start);
    while branches.len() > 0 {
        let mut new_branches = Vec::new();
        greatest_distance += 1;
        for branch in branches.iter() {
            let next_step = find_branches(&tiles, *branch);
            for (x, y) in next_step.iter(){
                if distances[*y][*x] == 0{
                    distances[*y][*x] = greatest_distance;
                    new_branches.push((*x, *y));
                }
            }
        }
        // print_highlighted(&tiles, &branches);
        branches = new_branches;
    }
    Some(greatest_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
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
    let mut distances = vec![vec![0; width]; height];
    let mut greatest_distance = 0;
    let mut branches = find_branches(&tiles, start);
    let mut start_tile : char = 'S';
    let mut above = false;
    let mut below = false;
    let mut left = false;
    let mut right = false;
    for (x, y) in branches.iter(){
        if x < &start.0 {
            left = true;
        }
        if x > &start.0 {
            right = true;
        }
        if y < &start.1 {
            above = true;
        }
        if y > &start.1 {
            below = true;
        }
    }
    if (true, true) == (above, left){
        start_tile = 'J';
    }
    if (true, true) == (above, right){
        start_tile = 'L';
    }
    if (true, true) == (below, left){
        start_tile = '7';
    }
    if (true, true) == (below, right){
        start_tile = 'F';
    }

    println!("Start Tile: {}", start_tile);
    let mut loop_tiles : Vec<(usize, usize)> = Vec::new();
    while branches.len() > 0 {
        let mut new_branches = Vec::new();
        greatest_distance += 1;
        for branch in branches.iter() {
            let next_step = find_branches(&tiles, *branch);
            for (x, y) in next_step.iter(){
                if distances[*y][*x] == 0{
                    distances[*y][*x] = greatest_distance;
                    new_branches.push((*x, *y));
                    loop_tiles.push((*x, *y));
                }
            }
        }
        // print_highlighted(&tiles, &branches);
        branches = new_branches;
    }
    let mut filtered_tiles = vec![vec!['.';tiles[0].len()]; tiles.len()];
    for (x, y) in loop_tiles.into_iter(){
        filtered_tiles[y][x] = tiles[y][x];
    }
    filtered_tiles[start.1][start.0] = start_tile;
    let mut enclosed_count = 0;
    for row in filtered_tiles.into_iter(){
        let mut cross_count = 0;
        let mut last_corner_tile = '.';
        for tile in row.into_iter(){
            if tile == '|'{
                cross_count += 1;
            } else if tile == '.' && cross_count % 2 == 1{
                enclosed_count += 1;
            } else {
                if tile == 'L' || tile == 'F'{
                    last_corner_tile = tile;
                } else if tile == '7' {
                    if last_corner_tile == 'L'{
                        cross_count += 1;
                    }
                } else if tile == 'J' {
                    if last_corner_tile == 'F'{
                        cross_count += 1;
                    }
                }
            }
        }
    }
    Some(enclosed_count)
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
