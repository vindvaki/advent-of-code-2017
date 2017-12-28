extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

fn main() {
}

type Node = (usize, usize);
type NeighborMap = HashMap<Node, HashSet<Node>>;

trait InsertAt {
    type Key;
    type Value;
    fn insert_at(&mut self, key: Self::Key, value: Self::Value) -> bool;
}

impl<K: PartialEq + Eq + Hash, V: PartialEq + Eq + Hash> InsertAt for HashMap<K, HashSet<V>> {
    type Key = K;
    type Value = V;
    fn insert_at(&mut self, key: Self::Key, value: Self::Value) -> bool {
        let created_new = false;
        if self.contains_key(&key) {
            let mut values = self.get_mut(&key).unwrap();
            values.insert(value);
            true
        } else {
            let mut values = HashSet::new();
            values.insert(value);
            self.insert(key, values);
            false
        }
    }
}

trait GraphTraversal {
    fn <Collection> traverse_graph(&self,  ) {
        let distance_map = HashMap<>;
    }

    fn bfs
}

fn parse_neighbor_map(input: &str) -> NeighborMap {
    let mut port_to_components = HashMap::new();

    for component_s in input.lines() {
        let component_vec: Vec<usize> = component_s.split('/').map(|s| s.parse::<usize>().unwrap()).collect();
        let component = (component_vec[0], component_vec[1]);

        port_to_components.insert_at(component.0.clone(), component.clone());
        port_to_components.insert_at(component.1.clone(), component.clone());
    }

    let mut neighbor_map = NeighborMap::new();

    for (_, ref overlapping) in port_to_components.iter() {
        for u in overlapping.iter() {
            for v in overlapping.iter() {
                if u != v {
                    neighbor_map.insert_at(u.clone(), v.clone());
                }
            }
        }
    }

    neighbor_map
}
