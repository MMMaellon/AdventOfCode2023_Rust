use std::collections::{HashMap, HashSet};
use std::thread;

advent_of_code::solution!(23);

fn parse(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|x| x.chars().collect()).collect();
}

#[derive(Clone)]
struct GraphNode {
    id: usize,
    x: usize,
    y: usize,
    edges: HashMap<usize, usize>,
}

fn parse_graph(input: &str) -> (Vec<GraphNode>, HashMap<(usize, usize), usize>) {
    let mut output = Vec::new();
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '#' {
                map.insert((j, i), output.len());
                output.push(GraphNode {
                    id: output.len(),
                    x: j,
                    y: i,
                    edges: HashMap::new(),
                });
            }
        }
    }
    return (output, map);
}

fn connect_graph(
    graph: &mut Vec<GraphNode>,
    tiles: &Vec<Vec<char>>,
    map: &mut HashMap<(usize, usize), usize>,
) {
    for (i, row) in tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile != '#' {
                if let Some(index) = map.get(&(j, i)) {
                    if j > 0 {
                        if let Some(other_index) = map.get(&(j - 1, i)) {
                            graph[*index].edges.insert(*other_index, 1);
                            graph[*other_index].edges.insert(*index, 1);
                        }
                    }
                    if let Some(other_index) = map.get(&(j + 1, i)) {
                        graph[*index].edges.insert(*other_index, 1);
                        graph[*other_index].edges.insert(*index, 1);
                    }
                    if i > 0 {
                        if let Some(other_index) = map.get(&(j, i - 1)) {
                            graph[*index].edges.insert(*other_index, 1);
                            graph[*other_index].edges.insert(*index, 1);
                        }
                    }
                    if let Some(other_index) = map.get(&(j, i + 1)) {
                        graph[*index].edges.insert(*other_index, 1);
                        graph[*other_index].edges.insert(*index, 1);
                    }
                }
            }
        }
    }
    let mut changed;
    loop {
        changed = false;
        for index_to_remove in 0..graph.len() {
            if graph[index_to_remove].edges.len() == 2 {
                changed = true;
                let mut cost_sum = 0;
                let owned_edges = graph[index_to_remove].edges.to_owned();
                let nodes_to_connect: Vec<&usize> = owned_edges.keys().collect();
                for (other_index, cost) in owned_edges.iter() {
                    cost_sum += cost;
                    let other = &mut graph[*other_index];
                    other.edges.remove(&index_to_remove);
                }
                graph[*nodes_to_connect[0]]
                    .edges
                    .insert(*nodes_to_connect[1], cost_sum);
                graph[*nodes_to_connect[1]]
                    .edges
                    .insert(*nodes_to_connect[0], cost_sum);
                graph[index_to_remove].edges.clear();
            }
        }

        if !changed {
            break;
        }
    }
}

fn naive_graph_walk(
    node_index: usize,
    target: (usize, usize),
    map: &HashMap<(usize, usize), usize>,
    graph: &Vec<GraphNode>,
    visited: &HashSet<(usize, usize)>,
) -> Option<usize> {
    // println!("Naive Graph Walk {}", node_index);
    let node = &graph[node_index];
    if visited.contains(&(node.x, node.y)) {
        return None;
    }
    if target == (node.x, node.y) {
        return Some(0);
    }
    let mut visited = visited.to_owned();
    visited.insert((node.x, node.y));

    let mut result: Option<usize> = None;
    for (&other_index, cost) in node.edges.iter() {
        // println!("{} checking -> {} | {}", node_index, other_index, cost);
        if let Some(value) = naive_graph_walk(other_index, target, map, graph, &visited) {
            if let Some(result_val) = result {
                result = Some(result_val.max(cost + value));
                // println!("{} result updated to {:?}", node_index, result);
            } else {
                result = Some(cost + value);
                // println!("{} result updated to {:?}", node_index, result);
            }
        }
    }

    return result;
}

fn longest_graph_paths(
    node_index: usize,
    target: (usize, usize),
    tiles: &Vec<Vec<char>>,
    map: &mut HashMap<(usize, usize), usize>,
    graph: &Vec<GraphNode>,
    memo: &mut Vec<usize>,
    visited: &HashSet<(usize, usize)>,
) -> Option<usize> {
    let node = &graph[node_index];
    if visited.contains(&(node.x, node.y)) {
        return None;
    }
    let mut visited = visited.to_owned();
    visited.insert((node.x, node.y));
    if target == (node.x, node.y) {
        memo[node.id] = 0;
        return Some(0);
    } else if node.edges.len() == 0 {
        return None;
    } else if node.edges.len() == 1 {
        let (other_index, cost) = node.edges.iter().next().unwrap();
        if let Some(rest_of_the_path_cost) =
            longest_graph_paths(*other_index, target, tiles, map, graph, memo, &visited)
        {
            memo[node.id] = memo[node.id].max(rest_of_the_path_cost + cost);
            return Some(rest_of_the_path_cost + cost);
        } else {
            return None;
        }
    }
    let mut lower_bound = 0;
    let mut highest_to_lowest = node
        .edges
        .to_owned()
        .into_iter()
        .collect::<Vec<(usize, usize)>>();
    highest_to_lowest.sort_by(|(_, cost_1), (_, cost_2)| cost_1.cmp(cost_2));
    for (other_index, cost) in highest_to_lowest.into_iter() {
        if lower_bound != 0 && (lower_bound < memo[other_index] || memo[other_index] == 0) {
            if let Some(rest_of_the_path) =
                longest_graph_paths(other_index, target, tiles, map, graph, memo, &visited)
            {
                lower_bound = lower_bound.max(rest_of_the_path + cost);
            }
        }
    }
    if lower_bound != 0 {
        memo[node.id] = memo[node.id].max(lower_bound);
        return Some(lower_bound);
    }
    return None;
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
            } else if left && memo[0][y][x] != (0, 0) {
                (x, y) = memo[0][y][x];
            } else if right && memo[1][y][x] != (0, 0) {
                (x, y) = memo[1][y][x];
            } else if up && memo[2][y][x] != (0, 0) {
                (x, y) = memo[2][y][x];
            } else if down && memo[3][y][x] != (0, 0) {
                (x, y) = memo[3][y][x];
            } else {
                x = next_x;
                y = next_y;
            }
            continue;
        }

        if looped {
            match prev_direction {
                Direction::Left => {
                    memo[0][prev_y][prev_x] = (x, y);
                }
                Direction::Right => {
                    memo[1][prev_y][prev_x] = (x, y);
                }
                Direction::Up => {
                    memo[2][prev_y][prev_x] = (x, y);
                }
                Direction::Down => {
                    memo[3][prev_y][prev_x] = (x, y);
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
    let (mut graph, mut map) = parse_graph(input);
    connect_graph(&mut graph, &tiles, &mut map);
    let start = find_start(&tiles);
    let end = find_end(&tiles);

    // for node in graph.iter() {
    //     if node.edges.is_empty() {
    //         continue;
    //     }
    //     println!("Node {} ({} {})", node.id, node.x, node.y);
    //     for edge in node.edges.iter() {
    //         println!(" -> {} {}", edge.0, edge.1);
    //     }
    // }

    // let mut memo = vec![vec![vec![(0, 0); tiles[0].len()]; tiles.len()]; 4];
    // let longest = longest_path_dry(&tiles, start, end, HashSet::new(), Vec::new(), &mut memo);
    let mut memo = vec![0; graph.len()];
    let visited: HashSet<(usize, usize)> = HashSet::new();
    let mut longest_paths: Vec<Option<usize>> = vec![None; graph.len()];
    let mut longest = None;
    // for node_index in (0..graph.len()).rev(){
    //     if graph[node_index].edges.len() == 2{
    //         continue;
    //     }
    //     longest_paths[node_index] = longest_graph_paths(node_index, end, &tiles, &mut map, &graph, &mut memo, &visited);
    //     if (graph[node_index].x, graph[node_index].y) == start{
    //         longest = longest_paths[node_index];
    //         break;
    //     }
    // }
    for node in graph.iter() {
        if (node.x, node.y) == start {
            longest = naive_graph_walk(node.id, end, &map, &graph, &visited);
        }
    }

    // println!("{:?}", longest_paths);

    // return Some(longest?.len());
    return longest;
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
