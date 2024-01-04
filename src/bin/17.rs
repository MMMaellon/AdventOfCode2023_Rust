use std::collections::{HashMap, HashSet};
use std::ops;

advent_of_code::solution!(17);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = Vec::new();
    for line in input.lines() {
        let mut new_row = Vec::new();
        for tile in line.trim().chars() {
            if let Ok(num) = tile.to_string().parse::<u32>() {
                new_row.push(num);
            }
        }
        output.push(new_row);
    }
    return output;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    const LEFT: Coordinate = Coordinate { x: -1, y: 0 };
    const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };
    const UP: Coordinate = Coordinate { x: 0, y: -1 };
    const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
    const ZERO: Coordinate = Coordinate { x: 0, y: 0 };
    fn to_index(self) -> usize {
        match self {
            Coordinate::LEFT => 0,
            Coordinate::RIGHT => 1,
            Coordinate::UP => 2,
            Coordinate::DOWN => 3,
            _ => 0,
        }
    }
}

impl ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    coord: Coordinate,
    loss: u32,
    chain_count: usize,
    prev: Coordinate,
    path: Vec<Coordinate>,
}

fn find_valid_neighbors(state: &State, tiles: &Vec<Vec<u32>>) -> Vec<State> {
    let mut fringe = Vec::new();
    let prev_difference;
    if state.prev == Coordinate::ZERO {
        prev_difference = Coordinate::ZERO;
    } else {
        prev_difference = state.coord - state.prev;
    }
    for neighbor_difference in [
        Coordinate::LEFT,
        Coordinate::RIGHT,
        Coordinate::UP,
        Coordinate::DOWN,
    ] {
        let neighbor_coord = state.coord + neighbor_difference;
        if neighbor_coord.x < 0
            || neighbor_coord.x > (tiles[0].len() - 1) as i32
            || neighbor_coord.y < 0
            || neighbor_coord.y > (tiles.len() - 1) as i32
            || state.path.contains(&neighbor_coord)
        {
            continue;
        }
        let neighbor_chain;
        if neighbor_difference == prev_difference || prev_difference == Coordinate::ZERO {
            if state.chain_count >= 2 {
                continue;
            } else {
                neighbor_chain = state.chain_count + 1;
            }
        } else {
            neighbor_chain = 0;
        }

        let neighbor_loss =
            state.loss + tiles[neighbor_coord.y as usize][neighbor_coord.x as usize];

        let mut new_state = State {
            coord: neighbor_coord,
            loss: neighbor_loss,
            chain_count: neighbor_chain,
            prev: state.coord,
            path: state.path.clone(),
        };
        new_state.path.push(state.coord);
        fringe.push(new_state);
    }
    return fringe;
}
fn find_valid_ultra_neighbors(state: &State, tiles: &Vec<Vec<u32>>) -> Vec<State> {
    let mut fringe = Vec::new();
    let prev_difference;
    prev_difference = state.coord - state.prev;
    let search_range;
    if state.chain_count < 4 && !state.path.is_empty(){
        search_range = vec![prev_difference];
        // println!("smaller search range {:?} chain_count {}", search_range, state.chain_count);
    } else {
        // println!("searching all neighbors of {} {}", state.coord.x, state.coord.y);
        search_range = vec![
            Coordinate::LEFT,
            Coordinate::RIGHT,
            Coordinate::UP,
            Coordinate::DOWN,
        ];
    }
    for neighbor_difference in search_range.into_iter() {
        let neighbor_coord = state.coord + neighbor_difference;
        if neighbor_coord.x < 0
            || neighbor_coord.x > (tiles[0].len() - 1) as i32
            || neighbor_coord.y < 0
            || neighbor_coord.y > (tiles.len() - 1) as i32
            || state.path.contains(&neighbor_coord)
        {
            continue;
        }
        let neighbor_chain;
        if neighbor_difference == prev_difference || state.path.is_empty() {
            if state.chain_count >= 10 {
                continue;
            } else {
                neighbor_chain = state.chain_count + 1;
            }
        } else {
            neighbor_chain = 1;
        }

        let neighbor_loss =
            state.loss + tiles[neighbor_coord.y as usize][neighbor_coord.x as usize];

        let mut new_state = State {
            coord: neighbor_coord,
            loss: neighbor_loss,
            chain_count: neighbor_chain,
            prev: state.coord,
            path: state.path.clone(),
        };
        new_state.path.push(state.coord);
        fringe.push(new_state);
    }
    return fringe;
}
fn find_ultra_least_loss_path(start: &State, tiles: &Vec<Vec<u32>>) -> u32 {
    let mut fringe = HashSet::<State>::new();
    let mut visited = vec![vec![vec![vec![0; 11]; 4]; tiles[0].len()]; tiles.len()];
    visited[start.coord.y as usize][start.coord.x as usize] = vec![vec![0; 3]; 4];
    for state in find_valid_ultra_neighbors(&start, &tiles) {
        fringe.insert(state.clone());
    }
    while !fringe.is_empty() {
        let mut least_in_fringe = 0;
        for state in fringe.iter() {
            if least_in_fringe == 0 || least_in_fringe > state.loss {
                least_in_fringe = state.loss;
            }
        }
        // println!("Fringe:");
        // for item in fringe.iter() {
        //     for print_item in item.path.iter() {
        //         print!("[{}, {}] > ", print_item.x, print_item.y);
        //     }
        //     print!("[{}, {}] - {}", item.coord.x, item.coord.y, item.loss);
        //     if item.loss == least_in_fringe {
        //         print!(" <-");
        //     }
        //     println!("");
        // }
        // println!("");
        let mut new_fringe = HashSet::new();
        for state in fringe.iter() {
            if state.loss == least_in_fringe {
                // println!("({}, {})", state.coord.x, state.coord.y);
                let direction = state.coord - state.prev;
                let prev_value = &mut visited[state.coord.y as usize][state.coord.x as usize]
                    [direction.to_index()][state.chain_count];
                if *prev_value != 0 && *prev_value <= state.loss {
                    // println!("discarding {} {} because prev value was {}", state.coord.x, state.coord.y, prev_value);
                    continue;
                }
                *prev_value = state.loss;
                if state.coord.x as usize == tiles[0].len() - 1
                    && state.coord.y as usize == tiles.len() - 1
                {
                    if state.chain_count < 4{
                        continue;
                    }
                    let mut total_loss = 0;
                    for print_item in state.path.iter() {
                        if *print_item != Coordinate::ZERO {
                            total_loss += tiles[print_item.y as usize][print_item.x as usize];
                        }
                        println!("({}, {}) - {}", print_item.x, print_item.y, total_loss);
                    }
                    return state.loss;
                }
                for neighbor in find_valid_ultra_neighbors(&state, &tiles) {
                    let prev_value = visited[neighbor.coord.y as usize][neighbor.coord.x as usize]
                        [(neighbor.coord - neighbor.prev).to_index()][neighbor.chain_count];
                    if prev_value == 0 || prev_value > neighbor.loss {
                        new_fringe.insert(neighbor);
                    }
                }
            }
        }
        fringe.retain(|x| x.loss > least_in_fringe);
        fringe.extend(new_fringe);
        // for row in visited.iter() {
        //     for tile in row.into_iter() {
        //         if tile.loss == 0 {
        //             print!("    ");
        //         } else {
        //             print!("{:0>3} ", tile.loss);
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
    }
    let mut least = 0;
    for v in visited[visited.len() - 1][visited[0].len() - 1].iter() {
        for x in v.iter() {
            if least == 0 || least > *x {
                least = *x;
            }
        }
    }
    return least;
}
fn find_least_loss_path(start: &State, tiles: &Vec<Vec<u32>>) -> u32 {
    let mut fringe = HashSet::<State>::new();
    let mut visited = vec![vec![vec![vec![0; 3]; 4]; tiles[0].len()]; tiles.len()];
    visited[start.coord.y as usize][start.coord.x as usize] = vec![vec![0; 3]; 4];
    for state in find_valid_neighbors(&start, &tiles) {
        fringe.insert(state.clone());
    }
    while !fringe.is_empty() {
        let mut least_in_fringe = 0;
        for state in fringe.iter() {
            if least_in_fringe == 0 || least_in_fringe > state.loss {
                least_in_fringe = state.loss;
            }
        }
        // println!("Fringe:");
        // for item in fringe.iter() {
        //     for print_item in item.path.iter() {
        //         print!("[{}, {}] > ", print_item.x, print_item.y);
        //     }
        //     print!("[{}, {}] - {}", item.coord.x, item.coord.y, item.loss);
        //     if item.loss == least_in_fringe {
        //         print!(" <-");
        //     }
        //     println!("");
        // }
        // println!("");
        let mut new_fringe = HashSet::new();
        for state in fringe.iter() {
            if state.loss == least_in_fringe {
                // println!("({}, {})", state.coord.x, state.coord.y);
                let direction = state.coord - state.prev;
                let prev_value = &mut visited[state.coord.y as usize][state.coord.x as usize]
                    [direction.to_index()][state.chain_count];
                if *prev_value != 0 && *prev_value <= state.loss {
                    // println!("discarding {} {} because prev value was {}", state.coord.x, state.coord.y, prev_value);
                    continue;
                }
                *prev_value = state.loss;
                if state.coord.x as usize == tiles[0].len() - 1
                    && state.coord.y as usize == tiles.len() - 1
                {
                    let mut total_loss = 0;
                    for print_item in state.path.iter() {
                        if *print_item != Coordinate::ZERO {
                            total_loss += tiles[print_item.y as usize][print_item.x as usize];
                        }
                        println!("({}, {}) - {}", print_item.x, print_item.y, total_loss);
                    }
                    return state.loss;
                }
                for neighbor in find_valid_neighbors(&state, &tiles) {
                    let prev_value = visited[neighbor.coord.y as usize][neighbor.coord.x as usize]
                        [(neighbor.coord - neighbor.prev).to_index()][neighbor.chain_count];
                    if prev_value == 0 || prev_value > neighbor.loss {
                        new_fringe.insert(neighbor);
                    }
                }
            }
        }
        fringe.retain(|x| x.loss > least_in_fringe);
        fringe.extend(new_fringe);
        // for row in visited.iter() {
        //     for tile in row.into_iter() {
        //         if tile.loss == 0 {
        //             print!("    ");
        //         } else {
        //             print!("{:0>3} ", tile.loss);
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
    }
    let mut least = 0;
    for v in visited[visited.len() - 1][visited[0].len() - 1].iter() {
        for x in v.iter() {
            if least == 0 || least > *x {
                least = *x;
            }
        }
    }
    return least;
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse_input(input);
    println!("part 1 width: {}, height = {}", tiles[0].len(), tiles.len());
    let least_loss = find_least_loss_path(
        &State {
            coord: Coordinate::ZERO,
            loss: 0,
            chain_count: 0,
            prev: Coordinate::ZERO,
            path: Vec::new(),
        },
        &tiles,
    );
    // let mut last_value = 0;
    // for _ in 0..100 {
    //     if last_value == 0 {
    //         last_value = find_least_loss_path(
    //             &State {
    //                 coord: Coordinate::ZERO,
    //                 loss: 0,
    //                 chain_count: 0,
    //                 prev: Coordinate::ZERO,
    //                 path: Vec::new(),
    //             },
    //             &tiles,
    //         );
    //     } else if last_value
    //         != find_least_loss_path(
    //             &State {
    //                 coord: Coordinate::ZERO,
    //                 loss: 0,
    //                 chain_count: 0,
    //                 prev: Coordinate::ZERO,
    //                 path: Vec::new(),
    //             },
    //             &tiles,
    //         )
    //     {
    //         println!("****************************");
    //         println!("****************************");
    //         println!("****************************");
    //         println!("****************************");
    //         println!("****************************");
    //         println!("***INCONSISTENCY DETECTED***");
    //         println!("****************************");
    //         return Some(least_loss);
    //     }
    // }
    return Some(least_loss);
}

pub fn part_two(input: &str) -> Option<u32> {
    let tiles = parse_input(input);
    println!("part 2 width: {}, height = {}", tiles[0].len(), tiles.len());
    let least_loss = find_ultra_least_loss_path(
        &State {
            coord: Coordinate::ZERO,
            loss: 0,
            chain_count: 0,
            prev: Coordinate::ZERO,
            path: Vec::new(),
        },
        &tiles,
    );
    return Some(least_loss);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(102));
    // }
    //
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
