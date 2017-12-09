extern crate aoc2017;

use std::collections::HashMap;

fn main() {
    let data = aoc2017::read_fixture();
    println!("part_1: {}", part_1(&data));
    println!("part_2: {}", part_2(&data));
}

fn parse_input(data: &str) -> (HashMap<String, i32>, HashMap<String, String>) {
    let mut weight_map = HashMap::new();
    let mut parent_map = HashMap::new();
    for line in data.lines() {
        let words: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')')
            .filter(|w| !w.is_empty())
            .collect();
        let parent = words[0];
        let weight: i32 = words[1].parse().unwrap();
        weight_map.insert(parent.to_owned(), weight);
        if words.len() > 2 {
            for i in 3..words.len() {
                parent_map.insert(words[i].to_owned(), parent.to_owned());
            }
        }
    }
    (weight_map, parent_map)
}

fn part_1(data: &str) -> String {
    let (weight_map, parent_map) = parse_input(&data);
    let mut k = weight_map.keys().nth(0).unwrap().clone();
    while parent_map.contains_key(&k) {
        k = parent_map[&k].clone();
    }
    k
}

fn part_2(data: &str) -> i32 {
    let (mut weight_map, parent_map) = parse_input(data);
    let mut child_map = HashMap::new();
    for (child, parent) in parent_map.iter() {
        if !child_map.contains_key(parent) {
            child_map.insert(parent.clone(), Vec::new());
        }
        child_map.get_mut(parent).unwrap().push(child.clone());
    }
    let root = part_1(data);
    find_bad_node(&root, &mut weight_map, &child_map).unwrap()
}

fn find_bad_node(
    node: &String,
    weight_map: &mut HashMap<String, i32>,
    child_map: &HashMap<String, Vec<String>>
) -> Option<i32> {
    if !child_map.contains_key(node) {
        return None;
    }
    let mut subtree_weight = 0;
    let mut weight_counts = HashMap::new();
    for child in child_map[node].iter() {
        if let Some(w) = find_bad_node(&child, weight_map, child_map) {
            return Some(w);
        }
        let curr_weight = weight_map[child];
        if !weight_counts.contains_key(&curr_weight) {
            weight_counts.insert(curr_weight, Vec::new());
        }
        weight_counts.get_mut(&curr_weight).unwrap().push(child.clone());
        subtree_weight += curr_weight;
    }
    if weight_counts.len() > 1 {
        let (culprit_weight, culprit_vec) = weight_counts.iter().min_by_key(|&(_, v)| v.len()).unwrap();
        let culprit = &culprit_vec[0];
        let target_weight = (subtree_weight - culprit_weight) / (child_map[node].len() as i32 - 1);
        let excess = culprit_weight - target_weight;
        let subweight: i32 = child_map[culprit].iter().map(|c| weight_map[c]).sum();
        return Some(culprit_weight - subweight - excess);
    }
    subtree_weight += weight_map[node];
    weight_map.insert(node.clone(), subtree_weight);
    None
}

#[cfg(test)]
mod tests {
    const DATA: &'static str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(part_2(&DATA), 60);
    }

    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1(&DATA), "tknk");
    }
}
