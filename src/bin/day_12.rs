extern crate aoc2017;
use aoc2017::read_fixture;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let graph = parse_graph(input);
    let mut visited = HashSet::new();
    subtree_size(0, &graph, &mut visited)
}

fn part_2(input: &str) -> usize {
    let graph = parse_graph(input);
    forest_size(&graph)
}

fn parse_graph(input: &str)-> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" <-> ");
        let u_str = parts.next().unwrap();
        let u: usize = u_str.parse().unwrap();
        for v_str in parts.next().unwrap().split(", ") {
            let v: usize = v_str.parse().unwrap();
            if !graph.contains_key(&u) {
                graph.insert(u, Vec::new());
            }
            if !graph.contains_key(&v) {
                graph.insert(v, Vec::new());
            }
            graph.get_mut(&u).unwrap().push(v);
            graph.get_mut(&v).unwrap().push(u);

        }
    }
    graph
}

fn subtree_size(root: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>) -> usize {
    if visited.contains(&root) || !graph.contains_key(&root) {
        return 0;
    }
    visited.insert(root);
    let child_tree_size_sum: usize = graph[&root].iter()
        .map(|&child| subtree_size(child, graph, visited))
        .sum();
    1 + child_tree_size_sum
}

fn forest_size(graph: &HashMap<usize, Vec<usize>>) -> usize {
    let mut visited = HashSet::new();
    let mut size = 0;
    for &k in graph.keys() {
        if !visited.contains(&k) {
            subtree_size(k, graph, &mut visited);
            size += 1;
        }
    }
    size
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(part_2(&input), 2);
    }
}
