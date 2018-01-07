extern crate aoc2017;
use aoc2017::read_fixture;

use std::collections::HashMap;
use std::fmt;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input, 16));
    println!("part_2: {}", part_2(&input, 16));
}

fn part_1(input: &str, len: usize) -> Dance {
    let mut dance = Dance::new(len);
    for op in input.split(",") {
        dance.apply(op.trim());
    }
    dance
}

fn part_2(input: &str, len: usize) -> String {
    let mut dance = Dance::new(len);
    let ops: Vec<&str> = input.split(",").map(|s| s.trim()).collect();
    let origin = dance.clone();
    let cycle_len = (1..).find(|_| {
        for op in ops.iter() {
            dance.apply(op);
        }
        dance == origin
    }).unwrap(); // safe because it finished
    for _ in 0..(1_000_000_000 % cycle_len) {
        for op in ops.iter() {
            dance.apply(op);
        }
    }
    format!("{}", dance)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Dance {
    len: usize,
    offset: usize,
    i2c: Vec<u32>,
    c2i: HashMap<u32, usize>,
}

impl fmt::Display for Dance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len {
            let c = std::char::from_u32(self.i2c[(self.len - self.offset + i) % self.len]).unwrap();
            write!(f, "{}", c)?;
        }
        write!(f, "{}", "")
    }
}

impl Dance {
    fn new(len: usize) -> Self {
        let mut res = Self {
            len: len,
            offset: 0,
            i2c: Vec::new(),
            c2i: HashMap::new(),
        };
        for (i, c) in (('a' as u32)..(('a' as u32) + (len as u32))).enumerate() {
            res.i2c.push(c);
            res.c2i.insert(c, i);
        }
        res
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.i2c.swap(i, j);
        self.c2i.insert(self.i2c[i], i);
        self.c2i.insert(self.i2c[j], j);
    }

    fn apply(&mut self, op: &str) {
        let split = op[1..].split('/');
        match &op[0..1] {
            "s" => {
                let add: usize = op[1..].parse().unwrap();
                self.offset = (self.offset + add) % self.len;
            },
            "x" => {
                let parts: Vec<usize> = split.map(|s| s.parse().unwrap()).collect();
                let i = (parts[0] + self.len - self.offset) % self.len;
                let j = (parts[1] + self.len - self.offset) % self.len;
                self.swap(i, j);
            },
            "p" => {
                let parts: Vec<u32> = split.map(|s| s.chars().next().unwrap() as u32).collect();
                let i = self.c2i[&parts[0]];
                let j = self.c2i[&parts[1]];
                self.swap(i, j);
            },
            _ => {},
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let s = format!("{}", part_1("s1,x3/4,pe/b\n", 5));
        assert_eq!(s, "baedc");
    }
}
