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

fn parse_component(input: &str) -> (usize, usize) {
    let mut iter = input
        .split('/')
        .map(|s| s.parse::<usize>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    (a, b)
}
