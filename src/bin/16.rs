use std::{collections::HashSet, ops::Range};

advent_of_code::solution!(16);

struct Tiles {
    width: usize,
    height: usize,
    x_vec: Vec<Vec<(usize, usize)>>,
    y_vec: Vec<Vec<(usize, usize)>>,
    chars: Vec<Vec<char>>,
}

impl Tiles {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            x_vec: Vec::new(),
            y_vec: Vec::new(),
            chars: Vec::new(),
        }
    }
}

fn parse_input(input: &str) -> Tiles {
    let mut output: Tiles = Tiles::new();
    for line in input.lines() {
        output.chars.push(line.chars().collect());
    }
    output.height = output.chars.len();
    if output.height == 0 {
        return output;
    }
    output.width = output.chars[0].len();
    output.x_vec = vec![Vec::new(); output.width];
    output.y_vec = vec![Vec::new(); output.height];
    for (y, row) in output.chars.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '/' | '\\' | '|' | '-' => {
                    output.x_vec[x].push((x, y));
                    output.y_vec[y].push((x, y));
                }
                _ => {}
            }
        }
    }
    return output;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Light {
    x: usize,
    y: usize,
    dir: Direction,
}

fn calc_start(start: &Light, tiles: &Tiles) -> Vec<Light> {
    let mut output_light = Vec::new();
    if tiles.chars.is_empty() || tiles.chars[0].is_empty() {
        return output_light;
    }

    match (start.dir,tiles.chars[start.y][start.y]) {
        (Direction::Right,'/') | (Direction::Left, '\\') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Up,
            });
        }
        (Direction::Right, '\\') | (Direction::Left, '/') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Down,
            });
        }
        (Direction::Right, '|') | (Direction::Left, '|') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Up,
            });
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Down,
            });
        }
        (Direction::Up,'/') | (Direction::Down, '\\') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Right,
            });
        }
        (Direction::Up, '\\') | (Direction::Down, '/') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Left,
            });
        }
        (Direction::Right, '-') | (Direction::Left, '-') => {
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Left,
            });
            output_light.push(Light {
                x: start.x,
                y: start.y,
                dir: Direction::Right,
            });
        }
        _ => {
            output_light.push(*start);
        }
    }
    return output_light;
}

fn step(light: &Light, tiles: &Tiles) -> Vec<Light> {
    let mut output_light = Vec::new();

    match &light.dir {
        Direction::Left => {
            for (tile_x, tile_y) in tiles.y_vec[light.y].iter().rev() {
                if *tile_x < light.x && tiles.chars[*tile_y][*tile_x] != '-' {
                    match tiles.chars[*tile_y][*tile_x] {
                        '/' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Down,
                            });
                        }
                        '\\' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Up,
                            });
                        }
                        '|' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Up,
                            });
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Down,
                            });
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
        Direction::Right => {
            for (tile_x, tile_y) in tiles.y_vec[light.y].iter() {
                if *tile_x > light.x && tiles.chars[*tile_y][*tile_x] != '-' {
                    match tiles.chars[*tile_y][*tile_x] {
                        '/' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Up,
                            });
                        }
                        '\\' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Down,
                            });
                        }
                        '|' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Up,
                            });
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Down,
                            });
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
        Direction::Up => {
            for (tile_x, tile_y) in tiles.x_vec[light.x].iter().rev() {
                if *tile_y < light.y && tiles.chars[*tile_y][*tile_x] != '|' {
                    match tiles.chars[*tile_y][*tile_x] {
                        '/' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Right,
                            });
                        }
                        '\\' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Left,
                            });
                        }
                        '-' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Left,
                            });
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Right,
                            });
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
        Direction::Down => {
            for (tile_x, tile_y) in tiles.x_vec[light.x].iter() {
                if *tile_y > light.y && tiles.chars[*tile_y][*tile_x] != '|' {
                    match tiles.chars[*tile_y][*tile_x] {
                        '/' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Left,
                            });
                        }
                        '\\' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Right,
                            });
                        }
                        '-' => {
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Left,
                            });
                            output_light.push(Light {
                                x: *tile_x,
                                y: *tile_y,
                                dir: Direction::Right,
                            });
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
    }
    return output_light;
}

fn print_energized(width: usize, height: usize, energized: &HashSet<(usize, usize)>) {
    let mut print_count = 0;
    for y in 0..height {
        for x in 0..width {
            if energized.contains(&(x, y)) {
                print!("#");
                print_count += 1;
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("count: {}", print_count);
    println!("energized: {}", energized.len());
    println!("");
}

fn calc_energized(start : &Light, tiles: &Tiles) -> usize{
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut calced_lights: HashSet<Light> = HashSet::new();
    let mut lights = calc_start(start, tiles);
    while !lights.is_empty() {
        let mut new_lights = Vec::new();
        for light in lights.iter() {
            let mut step_lights: Vec<Light> = Vec::new();
            let raw_step_lights = step(&light, &tiles);
            for &l in raw_step_lights.iter() {
                if !calced_lights.contains(&l) {
                    calced_lights.insert(l);
                    step_lights.push(l);
                }
            }

            let energized_range: Range<usize>;
            if !raw_step_lights.is_empty() {
                match light.dir {
                    Direction::Left => {
                        energized_range = raw_step_lights[0].x..(light.x + 1);
                    }
                    Direction::Right => {
                        energized_range = light.x..(raw_step_lights[0].x + 1);
                    }
                    Direction::Up => {
                        energized_range = raw_step_lights[0].y..(light.y + 1);
                    }
                    Direction::Down => {
                        energized_range = light.y..(raw_step_lights[0].y + 1);
                    }
                }
            } else {
                match light.dir {
                    Direction::Left => {
                        energized_range = 0..(light.x + 1);
                    }
                    Direction::Right => {
                        energized_range = light.x..tiles.width;
                    }
                    Direction::Up => {
                        energized_range = 0..(light.y + 1);
                    }
                    Direction::Down => {
                        energized_range = light.y..tiles.height;
                    }
                }
            }
            if light.dir == Direction::Left || light.dir == Direction::Right {
                for i in energized_range {
                    energized.insert((i, light.y));
                }
            } else {
                for i in energized_range {
                    energized.insert((light.x, i));
                }
            }
            new_lights.append(&mut step_lights);
        }
        lights.clear();
        lights.append(&mut new_lights);
    }
    return energized.len();
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse_input(input);
    Some(calc_energized(&Light{x : 0, y : 0, dir : Direction::Right}, &tiles) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tiles = parse_input(input);
    let mut most_energized = 0;
    for i in 0..tiles.height{
        most_energized = most_energized.max(calc_energized(&Light{x: 0, y: i, dir:Direction::Right}, &tiles));
    }
    for i in 0..tiles.height{
        most_energized = most_energized.max(calc_energized(&Light{x: tiles.width - 1 , y: i, dir:Direction::Left}, &tiles));
    }
    for i in 0..tiles.width{
        most_energized = most_energized.max(calc_energized(&Light{x: i, y: 0, dir:Direction::Down}, &tiles));
    }
    for i in 0..tiles.width{
        most_energized = most_energized.max(calc_energized(&Light{x: i, y: tiles.height - 1, dir:Direction::Up}, &tiles));
    }
    return Some(most_energized as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
