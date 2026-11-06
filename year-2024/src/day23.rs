use std::collections::{HashMap, HashSet};
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let (graph, edges) = parse_input("input/day23.txt")?;
    let part_one = find_cycles(&graph, &edges);
    let part_two = find_max_clique(&graph);

    println!("--- Day 23: LAN Party ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

type GraphAndEdges = (HashMap<String, HashSet<String>>, Vec<(String, String)>);

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<GraphAndEdges> {
    let lines = crate::read_lines(&filename)?;
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut edges = Vec::new();

    for line in lines.map_while(Result::ok) {
        let edge = line.trim().split_once('-').unwrap();
        let n1 = edge.0.to_string();
        let n2 = edge.1.to_string();
        graph.entry(n1.clone()).or_default().insert(n2.clone());
        graph.entry(n2.clone()).or_default().insert(n1.clone());
        edges.push((n1, n2));
    }

    Ok((graph, edges))
}

fn find_cycles(graph: &HashMap<String, HashSet<String>>, edges: &Vec<(String, String)>) -> usize {
    let mut cycles = 0;

    for edge in edges {
        if let (Some(n1), Some(n2)) = (graph.get(&edge.0), graph.get(&edge.1)) {
            let intersection = n1.intersection(n2);

            if edge.0.starts_with("t") || edge.1.starts_with("t") {
                cycles += intersection.count();
            } else {
                for node in intersection {
                    if node.starts_with("t") {
                        cycles += 1;
                    }
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
        .max_by_key(|&u| graph.get(u).map_or(0, |n| n.intersection(p).count()))?;
    let pivot_neighbors = &graph[pivot];
    let candidates: Vec<String> = p.difference(pivot_neighbors).cloned().collect();
    let mut max_clique: Option<HashSet<String>> = None;

    for node in candidates {
        let Some(neighbors) = graph.get(&node) else {
            continue;
        };

        let mut r_copy = r.clone();
        r_copy.insert(node.clone());
        let mut new_p: HashSet<String> = p.intersection(neighbors).cloned().collect();
        let mut new_x: HashSet<String> = x.intersection(neighbors).cloned().collect();
        let found = bron_kerbosch(&r_copy, &mut new_p, &mut new_x, graph);
        max_clique = [max_clique, found]
            .into_iter()
            .flatten()
            .max_by_key(HashSet::len);

        p.remove(&node);
        x.insert(node);
    }

    max_clique
}

fn find_max_clique(graph: &HashMap<String, HashSet<String>>) -> String {
    let mut nodes: HashSet<String> = graph.keys().cloned().collect();

    if let Some(max_clique) = bron_kerbosch(&HashSet::new(), &mut nodes, &mut HashSet::new(), graph)
    {
        let mut clique: Vec<String> = max_clique.iter().cloned().collect();
        clique.sort();
        return clique.join(",");
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let (graph, edges) = parse_input("input/day23-test.txt").unwrap();
        assert_eq!(find_cycles(&graph, &edges), 7);
    }

    #[test]
    fn part_two_example() {
        let (graph, _) = parse_input("input/day23-test.txt").unwrap();
        assert_eq!(find_max_clique(&graph), "co,de,ka,ta".to_string());
    }
}
