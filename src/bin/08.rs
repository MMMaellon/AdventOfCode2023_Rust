use std::collections::HashMap;
use num_integer::Integer;

advent_of_code::solution!(8);

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new() -> Self {
        Self {
            left: String::new(),
            right: String::new(),
        }
    }
}

struct OptimizedNode {
    name: String,
    left: usize,
    right: usize,
}

impl OptimizedNode {
    fn new(name: String) -> Self {
        Self {
            name,
            left: 0,
            right: 0,
        }
    }
}

#[derive(Debug)]
struct Path {
    looped: usize,
}

impl Path {
    fn new() -> Self {
        Self {
            looped: 0,
        }
    }
}

fn parse_input(input: &str, nodes: &mut HashMap<String, Node>) -> String {
    let mut instructions = String::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            instructions = line.trim().to_string();
            continue;
        } else if line.trim().is_empty() {
            continue;
        }

        if let Some((name, connections)) = line.split_once(" = (") {
            if let Some((left, right)) = connections.split_once(", ") {
                let node = nodes
                    .entry(name.trim().to_string())
                    .or_insert_with(Node::new);
                node.left = left[..left.len()].to_string();
                node.right = right[..right.len() - 1].to_string();
            }
        }
    }
    return instructions;
}
fn hash_to_vec(nodes: &HashMap<String, Node>) -> Vec<OptimizedNode> {
    let mut vec = Vec::with_capacity(nodes.len());
    let mut lookup: HashMap<String, usize> = HashMap::new();

    for (i, key) in nodes.keys().enumerate() {
        vec.push(OptimizedNode::new(key.clone()));
        lookup.insert(key.clone(), i);
    }

    for key in nodes.keys() {
        let node = &mut vec[lookup[key]];
        node.left = lookup[&nodes[key].left];
        node.right = lookup[&nodes[key].right];
    }

    return vec;
}
// fn step(direction: char, nodes: &HashMap<String, Node>, start: &String) -> String {
//     match direction {
//         'L' => {
//             return nodes.get(start).unwrap().left.clone();
//         }
//         'R' => {
//             return nodes.get(start).unwrap().right.clone();
//         }
//         _ => {
//             panic!("invalid character");
//         }
//     }
// }
fn step_vec(direction: char, nodes: &Vec<OptimizedNode>, start: usize) -> usize {
    match direction {
        'L' => {
            return nodes[start].left;
        }
        'R' => {
            return nodes[start].right;
        }
        _ => {
            panic!("invalid character");
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut nodes = HashMap::new();
    let instructions = parse_input(input, &mut nodes);
    let mut count: u32 = 0;
    let mut current_node: String = String::from("AAA");
    if !nodes.contains_key(&current_node) {
        return None;
    }
    while current_node != String::from("ZZZ") {
        for c in instructions.chars() {
            count += 1;
            match c {
                'L' => {
                    current_node = nodes.get(&current_node).unwrap().left.clone();
                }
                'R' => {
                    current_node = nodes.get(&current_node).unwrap().right.clone();
                }
                _ => {
                    panic!("invalid character");
                }
            }
            if current_node == "ZZZ" {
                break;
            }
        }

        if current_node == "ZZZ" {
            break;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut hash = HashMap::new();
    let instructions = parse_input(input, &mut hash);
    let nodes = hash_to_vec(&hash);
    let mut step_count: usize = 0;
    let mut starts: Vec<usize> = Vec::new();
    let mut paths: Vec<Path> = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        if node.name.ends_with("A") {
            starts.push(i);
            paths.push(Path::new());
        }
    }
    let mut looped_count = 0;
    while looped_count < starts.len() {
        for c in instructions.chars() {
            step_count += 1;
            for i in 0..starts.len() {
                if paths[i].looped == 0 {
                    starts[i] = step_vec(c, &nodes, starts[i]);
                    if nodes[starts[i]].name.ends_with("Z") {
                        looped_count += 1;
                        paths[i].looped = step_count;
                    }
                }
            }
        }
    }
    let mut least_common_multiple : u128 = 1;
    for path in paths.iter(){
        least_common_multiple = least_common_multiple.lcm(&(path.looped as u128));
    }
    println!("{:?}", paths);
    Some(least_common_multiple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6 as u128));
    }
}
