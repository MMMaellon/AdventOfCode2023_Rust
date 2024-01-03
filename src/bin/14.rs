use std::collections::HashMap;

advent_of_code::solution!(14);
fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut char_array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_array.push(line.chars().collect());
    }
    return char_array;
}
fn calc_north_weight(rocks: &Vec<Vec<char>>) -> usize {
    if rocks.is_empty() || rocks[0].is_empty() {
        return 0;
    }
    let mut last_blocker = vec![0; rocks[0].len()];
    let mut sum = 0;
    let row_count = rocks.len();
    for (i, row) in rocks.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            match c {
                '#' => {
                    last_blocker[j] = i + 1;
                }
                'O' => {
                    let weight = ((row_count as i32) - (last_blocker[j] as i32)) as usize;
                    sum += weight;
                    last_blocker[j] += 1;
                }
                _ => {}
            }
        }
    }
    return sum;
}

fn find_rocks(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut rocks: Vec<(usize, usize)> = Vec::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            match c {
                'O' => {
                    rocks.push((x, y));
                }
                _ => {}
            }
        }
    }
    return rocks;
}
fn find_blocks(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut rocks: Vec<(usize, usize)> = Vec::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            match c {
                '#' => {
                    rocks.push((x, y));
                }
                _ => {}
            }
        }
    }
    return rocks;
}

fn gen_north_piles(
    width: usize,
    _: usize,
    blocks: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut piles: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0)]; width];
    for (x, y) in blocks.iter() {
        if *y == 0 {
            piles[*x][0].1 += 1;
        } else {
            piles[*x].push((*y, 1));
        }
    }
    return piles;
}
fn gen_south_piles(
    width: usize,
    height: usize,
    blocks: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut piles: Vec<Vec<(usize, usize)>> = vec![vec![(height - 1, 0)]; width];
    for (x, y) in blocks.iter() {
        if *y == height - 1 {
            piles[*x][0].1 += 1;
        } else {
            piles[*x].push((*y, 1));
        }
    }
    return piles;
}
fn gen_west_piles(
    _width: usize,
    height: usize,
    blocks: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut piles: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0)]; height];
    for (x, y) in blocks.iter() {
        if *x == 0 {
            piles[*y][0].1 += 1;
        } else {
            piles[*y].push((*x, 1));
        }
    }
    return piles;
}
fn gen_east_piles(
    width: usize,
    height: usize,
    blocks: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut piles: Vec<Vec<(usize, usize)>> = vec![vec![(width - 1, 0)]; height];
    for (x, y) in blocks.iter() {
        if *x == width - 1 {
            piles[*y][0].1 += 1;
        } else {
            piles[*y].push((*x, 1));
        }
    }
    return piles;
}

fn move_north(
    piles: &Vec<Vec<(usize, usize)>>,
    _width: usize,
    _: usize,
    rocks: &mut Vec<(usize, usize)>,
    _blocks: &Vec<(usize, usize)>,
) {
    let mut piles = piles.clone();
    for rock in rocks.iter_mut() {
        let mut best_y = 0;
        let mut best_index = 0;
        for (i, (py, _)) in piles[rock.0].iter().enumerate() {
            if py > &best_y && py <= &rock.1 {
                best_y = *py;
                best_index = i;
            }
        }
        let best_pile = piles[rock.0][best_index];
        rock.1 = best_pile.0 + best_pile.1;
        piles[rock.0][best_index].1 += 1;
    }
}
fn move_south(
    piles: &Vec<Vec<(usize, usize)>>,
    _width: usize,
    height: usize,
    rocks: &mut Vec<(usize, usize)>,
    _blocks: &Vec<(usize, usize)>,
) {
    let mut piles = piles.clone();
    for rock in rocks.iter_mut() {
        let mut best_y = height - 1;
        let mut best_index = 0;
        for (i, (py, _)) in piles[rock.0].iter().enumerate() {
            if py < &best_y && py >= &rock.1 {
                best_y = *py;
                best_index = i;
            }
        }
        let best_pile = piles[rock.0][best_index];
        rock.1 = best_pile.0 - best_pile.1;
        piles[rock.0][best_index].1 += 1;
    }
}

fn move_west(
    piles: &Vec<Vec<(usize, usize)>>,
    _: usize,
    _height: usize,
    rocks: &mut Vec<(usize, usize)>,
    _blocks: &Vec<(usize, usize)>,
) {
    let mut piles = piles.clone();
    for rock in rocks.iter_mut() {
        let mut best_x = 0;
        let mut best_index = 0;
        for (i, (px, _)) in piles[rock.1].iter().enumerate() {
            if px > &best_x && px <= &rock.0 {
                best_x = *px;
                best_index = i;
            }
        }
        let best_pile = piles[rock.1][best_index];
        rock.0 = best_pile.0 + best_pile.1;
        piles[rock.1][best_index].1 += 1;
    }
}
fn move_east(
    piles: &Vec<Vec<(usize, usize)>>,
    width: usize,
    _height: usize,
    rocks: &mut Vec<(usize, usize)>,
    _blocks: &Vec<(usize, usize)>,
) {
    let mut piles = piles.clone();
    for rock in rocks.iter_mut() {
        let mut best_x = width;
        let mut best_index = 0;
        for (i, (px, _)) in piles[rock.1].iter().enumerate() {
            if px < &best_x && px >= &rock.0 {
                best_x = *px;
                best_index = i;
            }
        }
        let best_pile = piles[rock.1][best_index];
        rock.0 = best_pile.0 - best_pile.1;
        piles[rock.1][best_index].1 += 1;
    }
}
fn generate_hash(input: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (x, y) in input.into_iter() {
        sum += x + 1000 * y;
    }
    return sum;
}
fn cycle(
    width: usize,
    height: usize,
    rocks: &mut Vec<(usize, usize)>,
    blocks: &Vec<(usize, usize)>,
    north_pile: &Vec<Vec<(usize, usize)>>,
    south_pile: &Vec<Vec<(usize, usize)>>,
    west_pile: &Vec<Vec<(usize, usize)>>,
    east_pile: &Vec<Vec<(usize, usize)>>,
    memo: &mut HashMap<usize, Vec<(usize, usize)>>,
) -> bool {
    let key = generate_hash(&rocks);
    if let Some(new_rocks) = memo.get(&key) {
        *rocks = (*new_rocks).clone();
        // println!("we got it {}", memo.len());
        return false;
    }
    move_north(north_pile, width, height, rocks, blocks);
    move_west(west_pile, width, height, rocks, blocks);
    move_south(south_pile, width, height, rocks, blocks);
    move_east(east_pile, width, height, rocks, blocks);
    memo.insert(key, rocks.clone());
    return true;
}
fn calc_weight(height: usize, rocks: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (_, y) in rocks.into_iter() {
        sum += height - *y;
    }
    return sum;
}

fn print_array(
    width: usize,
    height: usize,
    rocks: &Vec<(usize, usize)>,
    blocks: &Vec<(usize, usize)>,
) {
    for y in 0..height {
        for x in 0..width {
            if rocks.contains(&(x, y)) {
                print!("O");
            } else if blocks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let char_array = parse_input(input);
    let north = calc_north_weight(&char_array);
    Some(north as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_array = parse_input(input);
    let mut rocks = find_rocks(&char_array);
    let blocks = find_blocks(&char_array);
    let height = char_array.len();
    let width = char_array[0].len();
    let mut memo = HashMap::new();
    let north_pile = gen_north_piles(width, height, &blocks);
    let south_pile = gen_south_piles(width, height, &blocks);
    let west_pile = gen_west_piles(width, height, &blocks);
    let east_pile = gen_east_piles(width, height, &blocks);
    let mut cycle_created: bool = false;
    let mut cycle_start: Vec<(usize, usize)> = Vec::new();
    let mut recorded_cycle: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut i : usize = 0;
    for index in 0..1000000000 {
        // for _ in 0..10000 {
        if !cycle(
            width,
            height,
            &mut rocks,
            &blocks,
            &north_pile,
            &south_pile,
            &west_pile,
            &east_pile,
            &mut memo,
        ) {
            if cycle_start.is_empty() {
                cycle_start = rocks.clone();
            } else {
                cycle_created = rocks == cycle_start;
                if cycle_created{
                    break;
                }
            }
            recorded_cycle.push(rocks.clone());
        }
    }
    if cycle_created{
        i = (1000000000 - memo.len() - 1) % recorded_cycle.len();
        println!("cycle len: {}, {}", recorded_cycle.len(), i);
        rocks = recorded_cycle[i].clone();
    }
    print_array(width, height, &rocks, &blocks);
    return Some(calc_weight(height, &rocks) as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
