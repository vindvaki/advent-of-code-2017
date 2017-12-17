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
    let origin = format!("{}", dance);
    let cycle_len = (1..).find(|_| {
        for op in ops.iter() {
            dance.apply(op);
        }
        let s = format!("{}", dance);
        s == origin
    }).unwrap(); // safe because it finished
    // reset dance
    dance = Dance::new(len);
    for _ in 0..(1_000_000_000 % cycle_len) {
        for op in ops.iter() {
            dance.apply(op);
        }
    }
    format!("{}", dance)
}

#[derive(Debug)]
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

    fn apply(&mut self, op: &str) {
        match &op[0..1] {
            "s" => {
                let add: usize = op[1..].parse().unwrap();
                self.offset += add;
                self.offset %= self.len;
            },
            "x" => {
                let mut parts = op[1..].split('/');
                let mut i: usize = parts.next().unwrap().parse().unwrap();
                let mut j: usize = parts.next().unwrap().parse().unwrap();
                i = (i + self.len - self.offset) % self.len;
                j = (j + self.len - self.offset) % self.len;
                self.i2c.swap(i, j);
                self.c2i.insert(self.i2c[i], i);
                self.c2i.insert(self.i2c[j], j);
            },
            "p" => {
                let mut parts = op[1..].split('/');
                let a: u32 = parts.next().unwrap().chars().next().unwrap() as u32;
                let b: u32 = parts.next().unwrap().chars().next().unwrap() as u32;
                let i = self.c2i[&a];
                let j = self.c2i[&b];
                self.i2c.swap(i, j);
                self.c2i.insert(a, j);
                self.c2i.insert(b, i);
            },
            _ => {},
        }
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

    #[test]
    fn test_part_2() {
        use part_2;
        let s = format!("{}", part_2("s1,x3/4,pe/b\n", 5));
        assert_eq!(s, "ceadb");
    }

}
