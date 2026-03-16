use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashMap;

pub fn main() -> Result<(), Error> {
    let path = "input/day11.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut graph = HashMap::new();

    for line in buffered.lines() {
        let line_ok = line?;

        let mut split_line = line_ok.trim().split_whitespace();
        let node = String::from(split_line.next().unwrap().trim_matches(|c| c == ':'));
        let neighbours: Vec<String> = split_line.map(|x| x.to_string()).collect();

        graph.insert(node, neighbours);
    }

    let mut cache_one = HashMap::new();
    let nodes_to_visit = ["svr", "fft", "dac", "out"].to_vec();

    let part_one = traverse_graph("you".to_string(), "out".to_string(), &graph, &mut cache_one);
    let part_two = must_visit(nodes_to_visit, &graph);

    println!("--- Day 11: Reactor ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn traverse_graph(
    current_node: String,
    end_node: String,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if current_node == end_node {
        return 1;
    }

    if current_node == "out".to_string() {
        return 0;
    }

    if let Some(&distance) = cache.get(&current_node) {
        return distance;
    }

    let mut total_paths = 0;
    let neigbours = graph.get(&current_node).unwrap();

    for neigbour in neigbours {
        let current_paths = traverse_graph(neigbour.to_string(), end_node.clone(), graph, cache);
        total_paths += current_paths;
    }

    cache.insert(current_node, total_paths);

    total_paths
}

fn must_visit(nodes: Vec<&str>, graph: &HashMap<String, Vec<String>>) -> usize {
    let mut total_paths = 1;
    let n = nodes.len();

    for i in 0..n - 1 {
        let start = nodes[i].to_string();
        let end = nodes[i + 1].to_string();
        let mut cache = HashMap::new();

        total_paths *= traverse_graph(start, end, graph, &mut cache);
    }

    total_paths
}
