use std::{
    cmp::{max, min},
    usize,
};

advent_of_code::solution!(11);

fn parse_to_chars(input: &str) -> Vec<Vec<char>> {
    let mut chars = Vec::new();
    for line in input.lines() {
        chars.push(line.chars().collect());
    }
    return chars;
}
fn find_empty_columns(input: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_columns: Vec<usize> = Vec::new();
    let mut empty = vec![true; input[0].len()];
    for (i, column) in input.into_iter().enumerate() {
        for (j, c) in column.into_iter().enumerate() {
            empty[j] = empty[j] && *c == '.';
        }
    }
    for (i, tile) in empty.into_iter().enumerate() {
        if tile {
            empty_columns.push(i);
        }
    }
    return empty_columns;
}

fn find_empty_rows(input: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = Vec::new();
    for (i, row) in input.into_iter().enumerate() {
        let mut empty = false;
        for c in row.into_iter() {
            empty = *c == '.';
            if !empty {
                break;
            }
        }
        if empty {
            empty_rows.push(i);
        }
    }
    return empty_rows;
}

fn expand(chars: &Vec<Vec<char>>, empty_rows: &[usize], empty_columns: &[usize]) -> Vec<Vec<char>> {
    let mut expanded_vec = Vec::new();
    let empty_row_len = chars[0].len() + empty_columns.len();
    for (i, row) in chars.into_iter().enumerate() {
        if empty_rows.contains(&i) {
            expanded_vec.push(vec!['.'; empty_row_len]);
            expanded_vec.push(vec!['.'; empty_row_len]);
        } else {
            let mut expanded_row = Vec::new();
            for (j, col) in row.iter().enumerate() {
                if empty_columns.contains(&j) {
                    expanded_row.push(*col);
                }
                expanded_row.push(*col);
            }
            expanded_vec.push(expanded_row);
        }
    }
    return expanded_vec;
}

fn expand_galaxy((x, y) : (usize, usize), empty_rows : &Vec<usize>, empty_columns: &Vec<usize>) -> (u128, u128){
    let mut fewer_rows: u128 = 0;
    let mut fewer_cols: u128 = 0;
    let expand_factor : u128= 1000000 - 1;
    for row in empty_rows.iter(){
        if row < &y{
            fewer_rows += 1;
        }
    }
    for col in empty_columns.iter(){
        if col < &x{
            fewer_cols += 1;
        }
    }
    ((x as u128) + (fewer_cols * expand_factor), (y as u128) + (fewer_rows * expand_factor))
}

fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    return max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2);
}

fn long_distance((x1, y1): (u128, u128), (x2, y2): (u128, u128)) -> u128 {
    return max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2);
}

fn print_row(row: &Vec<char>){
    for c in row{
        print!("{}", c);
    }
    println!("");
}

fn find_galaxies(chars: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (y, row) in chars.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if *c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    return galaxies;
}

pub fn part_one(input: &str) -> Option<u128> {
    let chars = parse_to_chars(input);
    let empty_rows = find_empty_rows(&chars);
    let empty_columns = find_empty_columns(&chars);
    let expanded_chars = expand(&chars, &empty_rows, &empty_columns);
    let galaxies = find_galaxies(&expanded_chars);
    let mut sum: usize = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let dist = distance(galaxies[i], galaxies[j]);
            sum += dist;
        }
    }

    Some(sum as u128)
}

pub fn part_two(input: &str) -> Option<u128> {
    let chars = parse_to_chars(input);
    let empty_rows = find_empty_rows(&chars);
    let empty_columns = find_empty_columns(&chars);
    let galaxies : Vec<(u128, u128)> = find_galaxies(&chars).into_iter().map(|x| expand_galaxy(x, &empty_rows, &empty_columns)).collect();
    let mut sum: u128 = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let dist = long_distance(galaxies[i], galaxies[j]);
            sum += dist;
        }
    }
    Some(sum as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
