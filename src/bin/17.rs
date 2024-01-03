use std::collections::HashSet;

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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    x: usize,
    y: usize,
    loss: u32,
    dir: Direction,
    prev_count: usize,
    prev: Vec<(usize, usize)>,
}

fn find_valid_neighbors(state: &State, tiles: &Vec<Vec<u32>>) -> Vec<State> {
    let mut fringe = Vec::new();
    //Left
    if state.x > 0 && (state.dir != Direction::Left || state.prev_count < 2) {
        if state.dir == Direction::Left {
            let mut new_neighbor = State {
                x: state.x - 1,
                y: state.y,
                loss: state.loss + tiles[state.y][state.x - 1],
                dir: Direction::Left,
                prev_count: state.prev_count + 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        } else {
            let mut new_neighbor = State {
                x: state.x - 1,
                y: state.y,
                loss: state.loss + tiles[state.y][state.x - 1],
                dir: Direction::Left,
                prev_count: 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        }
    }
    //Right
    if state.x < tiles[0].len() - 1 && (state.dir != Direction::Right || state.prev_count < 2) {
        if state.dir == Direction::Right {
            let mut new_neighbor = State {
                x: state.x + 1,
                y: state.y,
                loss: state.loss + tiles[state.y][state.x + 1],
                dir: Direction::Right,
                prev_count: state.prev_count + 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        } else {
            let mut new_neighbor = State {
                x: state.x + 1,
                y: state.y,
                loss: state.loss + tiles[state.y][state.x + 1],
                dir: Direction::Right,
                prev_count: 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        }
    }
    //Up
    if state.y > 0 && (state.dir != Direction::Up || state.prev_count < 2) {
        if state.dir == Direction::Up {
            let mut new_neighbor = State {
                x: state.x,
                y: state.y - 1,
                loss: state.loss + tiles[state.y - 1][state.x],
                dir: Direction::Up,
                prev_count: state.prev_count + 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        } else {
            let mut new_neighbor = State {
                x: state.x,
                y: state.y - 1,
                loss: state.loss + tiles[state.y - 1][state.x],
                dir: Direction::Up,
                prev_count: 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        }
    }
    //Down
    if state.y < tiles.len() - 1 && (state.dir != Direction::Down || state.prev_count < 2) {
        if state.dir == Direction::Down {
            let mut new_neighbor = State {
                x: state.x,
                y: state.y + 1,
                loss: state.loss + tiles[state.y + 1][state.x],
                dir: Direction::Down,
                prev_count: state.prev_count + 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        } else {
            let mut new_neighbor = State {
                x: state.x,
                y: state.y + 1,
                loss: state.loss + tiles[state.y + 1][state.x],
                dir: Direction::Down,
                prev_count: 1,
                prev: state.prev.clone(),
            };
            new_neighbor.prev.push((state.x, state.y));
            fringe.push(new_neighbor);
        }
    }
    return fringe;
}

fn find_least_loss_path(start: &State, tiles: &Vec<Vec<u32>>) -> u32 {
    let mut fringe = HashSet::<State>::new();
    let mut visited = vec![vec![0; tiles[0].len()]; tiles.len()];
    visited[start.y][start.x] = 1;
    for state in find_valid_neighbors(&start, &tiles) {
        fringe.insert(state.clone());
        visited[state.y][state.x] = state.loss;
    }
    while !fringe.is_empty() {
        let mut least_in_fringe = 0;
        for state in fringe.iter() {
            if least_in_fringe == 0 || least_in_fringe > state.loss {
                least_in_fringe = state.loss;
            }
        }
        let mut new_fringe = HashSet::new();
        for state in fringe.iter() {
            if state.loss == least_in_fringe {
                if state.x == tiles[0].len() - 1 && state.y == tiles.len() - 1 {
                    for (x, y) in state.prev.iter(){
                        println!("({x}, {y})");
                    }
                    return state.loss;
                }
                for neighbor in find_valid_neighbors(&state, &tiles) {
                    if visited[neighbor.y][neighbor.x] == 0
                        || visited[neighbor.y][neighbor.x] > neighbor.loss
                    {
                        visited[neighbor.y][neighbor.x] = neighbor.loss;
                        new_fringe.insert(neighbor);
                    }
                }
            } else {
                new_fringe.insert(state.clone());
            }
        }
        fringe = new_fringe;
        // for row in visited.iter() {
        //     for tile in row.into_iter() {
        //         if *tile == 0 {
        //             print!("    ");
        //         } else {
        //             print!("{tile:0>3} ");
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
    }
    return visited[tiles[0].len() - 1][tiles.len() - 1];
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse_input(input);
    let least_loss = find_least_loss_path(
        &State {
            x: 0,
            y: 0,
            loss: 0,
            dir: Direction::Right,
            prev_count: 0,
            prev: Vec::new(),
        },
        &tiles,
    );
    return Some(least_loss);
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
