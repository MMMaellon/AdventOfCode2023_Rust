use std::collections::{BTreeSet, HashMap, HashSet};
use std::thread;

advent_of_code::solution!(25);

#[derive(Clone)]
struct Node {
    connection_strings: HashSet<String>,
    connections: HashSet<usize>,
    name: String,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            connection_strings: HashSet::new(),
            connections: HashSet::new(),
            name,
        }
    }
}

#[derive(Clone)]
struct Graph {
    map: HashMap<String, usize>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            nodes: Vec::new(),
        }
    }
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        if let Some((name, connections)) = line.split_once(": ") {
            let mut node = Node::new(name.to_string());
            node.connection_strings = connections
                .trim()
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            graph.map.insert(node.name.clone(), graph.nodes.len());
            graph.nodes.push(node);
        }
    }

    for index in 0..graph.nodes.len() {
        for connection_str in graph.nodes[index].connection_strings.to_owned().iter() {
            if graph.map.contains_key(connection_str) {
                graph.nodes[index]
                    .connections
                    .insert(graph.map[connection_str]);
            } else {
                graph
                    .map
                    .insert(connection_str.to_string(), graph.nodes.len());
                graph.nodes.push(Node::new(connection_str.to_owned()));
            }
        }
        for i in graph.nodes[index].connections.to_owned().into_iter() {
            graph.nodes[i].connections.insert(index);
        }
    }

    return graph;
}

fn count_edges(graph: &Graph, split: &HashSet<usize>) -> usize {
    let mut count = 0;
    for node_index in split.iter() {
        let node = &graph.nodes[*node_index];
        for edge in node.connections.iter() {
            if !split.contains(edge) {
                count += 1;
            }
        }
    }
    return count;
}

fn find_split(
    graph: &Graph,
    split: &HashSet<usize>,
    edges: &HashSet<usize>,
    best: &mut HashSet<usize>,
    memo: &mut HashSet<BTreeSet<usize>>,
) {
    // println!("find_split - {} {:?}", edges.len(), split);
    if best.len() > 0 {
        return;
    }
    if split.len() == graph.nodes.len() {
        return;
    }
    let btree = split.clone().into_iter().collect();
    if memo.contains(&btree) {
        // println!("btree return");
        return;
    }
    memo.insert(btree);
    let best_score = best.len() * (graph.nodes.len() - best.len());
    let split_score = split.len() * (graph.nodes.len() - split.len());
    // println!("best {} split {}", best_score, split_score);
    if best_score > split_score {
        return;
    }
    if count_edges(graph, split) == 3 && split_score > best_score {
        *best = split.to_owned();
    }
    for edge in edges.iter() {
        let mut new_edges = edges.clone();
        let mut new_split = split.clone();
        new_split.insert(*edge);
        new_edges.remove(edge);
        for new_edge in graph.nodes[*edge].connections.iter() {
            if !new_split.contains(new_edge) {
                new_edges.insert(*new_edge);
            }
        }
        find_split(graph, &new_split, &new_edges, best, memo);
    }
}
fn find_split_rev(graph: &Graph) -> HashSet<usize> {
    let mut split: HashSet<usize> = (0..graph.nodes.len()).collect();
    split.remove(&1);
    let mut count = 0;
    let mut max_count;
    let mut max_index = 0;
    while count != 3 {
        count = 0;
        max_count = 0;
        for node_index in split.iter() {
            let node = &graph.nodes[*node_index];
            let mut node_count = 0;

            for edge in node.connections.iter() {
                if !split.contains(edge) {
                    node_count += 1;
                    count += 1;
                }
            }
            if node_count > max_count {
                max_count = node_count;
                max_index = *node_index;
            }
        }
        if count == 3 || !split.remove(&max_index){
            println!("exit early {}", count);
            break;
        }
    }
    return split;
}

fn part_one_thread(input: &str) -> Option<usize> {
    let graph = parse(input);
    let mut best = HashSet::new();
    let mut split = HashSet::new();
    let mut edges = HashSet::new();
    // let mut memo = HashSet::new();
    split.insert(0);
    for edge in graph.nodes[0].connections.iter() {
        edges.insert(*edge);
    }
    // find_split(&graph, &split, &edges, &mut best, &mut memo);
    best = find_split_rev(&graph);
    let best_score = best.len() * (graph.nodes.len() - best.len());
    return Some(best_score);
}

const STACK_SIZE: usize = 4 * 1024 * 1024;

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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
