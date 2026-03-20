use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Error> {
    // let path = "input/day23-test.txt";
    let path = "input/day23.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut edges = Vec::new();

    for line in buffered.lines() {
        let line_ok = line?;
        let edge = line_ok.trim().split_once('-').unwrap();
        let node_left = edge.0.to_string();
        let node_right = edge.1.to_string();

        graph
            .entry(node_left.clone())
            .or_insert_with(HashSet::new)
            .insert(node_right.clone());

        graph
            .entry(node_right.clone())
            .or_insert_with(HashSet::new)
            .insert(node_left.clone());

        edges.push((node_left, node_right));
    }

    let part_one = find_cycles(&graph, &edges);
    let part_two = find_max_clique(&graph);

    println!("--- Day 23: LAN Party ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn find_cycles(graph: &HashMap<String, HashSet<String>>, edges: &Vec<(String, String)>) -> u64 {
    let mut cycles = 0;

    for edge in edges {
        if let (Some(n1), Some(n2)) = (graph.get(&edge.0), graph.get(&edge.1)) {
            let intersect = n1.intersection(&n2);

            for node in intersect {
                if edge.0.starts_with("t") || edge.1.starts_with("t") || node.starts_with("t") {
                    cycles += 1;
                }
            }
        }
    }

    cycles / 3
}

fn bron_kerbosch(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
) -> Option<HashSet<String>> {
    if p.is_empty() && x.is_empty() {
        return Some(r.clone());
    }

    let pivot = p
        .iter()
        .chain(x.iter())
        .max_by_key(|u| {
            graph
                .get(*u)
                .map(|neighbors| neighbors.intersection(p).count())
                .unwrap_or(0)
        })
        .unwrap()
        .clone();

    let pivot_neighbors = graph
        .get(&pivot)
        .unwrap_or_else(|| panic!("node '{}' not found in graph", pivot));

    let candidates: Vec<String> = p.difference(pivot_neighbors).cloned().collect();

    let mut max_clique: Option<HashSet<String>> = None;
    let mut max_clique_size = 0;

    for node in candidates {
        if let Some(neighbours) = graph.get(&node) {
            let mut new_r = r.clone();
            new_r.insert(node.clone());

            let mut new_p: HashSet<String> = p.intersection(neighbours).cloned().collect();
            let mut new_x: HashSet<String> = x.intersection(neighbours).cloned().collect();

            let clique = bron_kerbosch(&new_r, &mut new_p, &mut new_x, graph);

            match clique {
                Some(c) => {
                    let n = c.len();
                    if n > max_clique_size {
                        max_clique_size = n;
                        max_clique = Some(c);
                    }
                }
                None => continue,
            }

            p.remove(&node);
            x.insert(node);
        }
    }

    max_clique
}

fn find_max_clique(graph: &HashMap<String, HashSet<String>>) -> String {
    let mut nodes: HashSet<String> = graph.keys().cloned().collect();

    let max_clique = bron_kerbosch(&HashSet::new(), &mut nodes, &mut HashSet::new(), graph);

    match max_clique {
        Some(c) => {
            let mut c_vec: Vec<String> = c.iter().cloned().collect();
            c_vec.sort();
            return c_vec.join(",");
        }
        None => unreachable!("Should never happen!"),
    }
}
