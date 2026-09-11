use std::collections::HashMap;
use std::io;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let graph = parse_input("input/day11.txt")?;
    let start = "you".to_string();
    let end = "out".to_string();
    let mut cache = HashMap::new();
    let dac_to_fft = ["svr", "dac", "fft", "out"].to_vec();
    let fft_to_dac = ["svr", "fft", "dac", "out"].to_vec();
    let part_one = traverse_graph(start, end, &graph, &mut cache);
    let part_two =
        traverse_conditional(dac_to_fft, &graph) + traverse_conditional(fft_to_dac, &graph);

    println!("--- Day 11: Reactor ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}\n", part_two);

    Ok(())
}

fn parse_input<P: AsRef<Path>>(filename: P) -> io::Result<HashMap<String, Vec<String>>> {
    let lines = crate::read_lines(filename)?;
    let mut graph = HashMap::new();

    for line in lines.map_while(Result::ok) {
        let mut line = line.split_whitespace();
        let node = String::from(line.next().unwrap().trim_matches(|c| c == ':'));
        let neighbours = line.map(|s| s.to_string()).collect();
        graph.insert(node, neighbours);
    }

    Ok(graph)
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

    if current_node == "out" {
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

fn traverse_conditional(nodes: Vec<&str>, graph: &HashMap<String, Vec<String>>) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let graph = parse_input("input/day11-test1.txt").unwrap();
        let start = "you".to_string();
        let end = "out".to_string();
        let mut cache: HashMap<String, usize> = HashMap::new();
        assert_eq!(traverse_graph(start, end, &graph, &mut cache), 5)
    }

    #[test]
    fn part_two_example() {
        let graph = parse_input("input/day11-test2.txt").unwrap();
        let dac_to_fft = ["svr", "dac", "fft", "out"].to_vec();
        let fft_to_dac = ["svr", "fft", "dac", "out"].to_vec();
        assert_eq!(
            traverse_conditional(dac_to_fft, &graph) + traverse_conditional(fft_to_dac, &graph),
            2
        )
    }
}
