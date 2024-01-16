use std::{
    cmp::{max, min},
    fmt,
    ops::RangeInclusive,
};
advent_of_code::solution!(18);

struct Step {
    x: i128,
    y: i128,
    color: u128,
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{({}, {}), {:06x}}}", self.x, self.y, self.color)
    }
}

fn parse(input: &str) -> Vec<Step> {
    let mut output = Vec::new();
    for line in input.lines() {
        if let Some((direction, remainder)) = line.split_once(' ') {
            if let Some((count_str, remainder)) = remainder.split_once(' ') {
                if let Ok(count) = count_str.parse::<i128>() {
                    if let Ok(color) = u128::from_str_radix(
                        remainder.trim_matches(|x: char| !x.is_alphanumeric()),
                        16,
                    ) {
                        let mut step = Step {
                            x: count,
                            y: 0,
                            color,
                        };
                        match direction {
                            "U" => {
                                step.y = -count;
                                step.x = 0;
                            }
                            "D" => {
                                step.y = count;
                                step.x = 0;
                            }
                            "L" => {
                                step.x = -count;
                            }
                            _ => {}
                        }
                        output.push(step);
                    }
                } else {
                    println!("bad parse {}", line);
                }
            } else {
                println!("bad count {}", line);
            }
        } else {
            println!("bad direction {}", line);
        }
    }
    return output;
}

fn parse_from_color(input : &Vec<Step>) -> Vec<Step>{
    let mut output = Vec::new();
    for step in input{
        let hex_string = format!("{:06x}", step.color);
        let mut new_step = Step{
            x: 0,
            y: 0,
            color: step.color,
        };
        match hex_string.get(hex_string.len() - 1..){
            Some("0") => { new_step.x = 1 }
            Some("1") => { new_step.y = 1 }
            Some("2") => { new_step.x = -1 }
            Some("3") => { new_step.y = -1 }
            _ => {}
        }
        if let Ok(value) = i128::from_str_radix(&hex_string[0..5], 16){
            new_step.x *= value;
            new_step.y *= value;
        }
        output.push(new_step);
    }
    return output;
}

fn calc_area(steps: &Vec<Step>) -> usize {
    let (negative_x, positive_x, negative_y, positive_y) = get_bounding_box(&steps);
    let mut height = negative_y.abs();
    let count: i128;
    let mut border: i128 = 1;
    let mut lefts: i128 = 0;
    let mut rights: i128 = 0;
    for step in steps.into_iter() {
        if step.y != 0 {
            height += step.y;
            //the vertical line
            if step.y > 0 {
                border += step.y;
            }
        } else {
            if step.x < 0 {
                lefts += height * step.x.abs();
            } else {
                rights += height * step.x.abs();
                border += step.x.abs();
            }
        }
    }
    if lefts > rights {
        count = border + lefts - rights;
    } else {
        count = border + rights - lefts;
    }
    return count.abs() as usize;
}
fn calc_perimeter(steps: &Vec<Step>) -> usize {
    let mut count = 0;
    for step in steps.into_iter() {
        if step.y != 0 {
            count += step.y.abs();
        } else {
            count += step.x.abs();
        }
    }
    return count as usize;
}

fn get_bounding_box(steps: &Vec<Step>) -> (i128, i128, i128, i128) {
    let mut positive_y = 0;
    let mut negative_y = 0;
    let mut positive_x = 0;
    let mut negative_x = 0;
    let mut x = 0;
    let mut y = 0;
    for step in steps.iter() {
        x += step.x;
        y += step.y;
        negative_x = min(negative_x, x);
        positive_x = max(positive_x, x);
        negative_y = min(negative_y, y);
        positive_y = max(positive_y, y);
    }
    return (negative_x, positive_x, negative_y, positive_y);
}

fn print_path(steps: &Vec<Step>) {
    let (negative_x, positive_x, negative_y, positive_y) = get_bounding_box(&steps);
    let height = (negative_y.abs() + positive_y) as usize + 1;
    let width = (negative_x.abs() + positive_x) as usize + 1;
    let mut grid = vec![vec!['.'; width]; height + 1];
    grid[negative_y.abs() as usize] = vec!['-'; width];
    let mut x = negative_x.abs();
    let mut y = negative_y.abs();
    for step in steps.iter() {
        let new_x = x + step.x;
        let new_y = y + step.y;
        for i in min(new_y, y)..=max(new_y, y) {
            for j in min(new_x, x)..=max(new_x, x) {
                if i >= negative_y.abs() {
                    grid[i as usize + 1][j as usize] = '#';
                } else {
                    grid[i as usize][j as usize] = '#';
                }
            }
        }
        x += step.x;
        y += step.y;
    }
    for line in grid.into_iter() {
        for tile in line.into_iter() {
            print!("{}", tile);
        }
        println!("");
    }
}
pub fn part_one(input: &str) -> Option<u128> {
    let steps = parse(input);
    print_path(&steps);
    // println!("perimeter {}", calc_perimeter(&steps));
    println!("area {}", calc_area(&steps));
    // println!("both {}", calc_perimeter(&steps) + calc_area(&steps));
    return Some(calc_area(&steps) as u128);
}

pub fn part_two(input: &str) -> Option<u128> {
    let steps = parse(input);
    let steps = parse_from_color(&steps);
    return Some(calc_area(&steps) as u128);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
