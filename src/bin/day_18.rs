extern crate aoc2017;
use aoc2017::read_fixture;

use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(input.as_str()));
}

fn part_1(input: &str) -> i64 {
    let mut player = Player::new(input);
    while player.last_rcv.is_none() || player.last_rcv == Some(0) {
        player.step();
    }
    player.last_rcv.unwrap()
}

struct Player {
    prog: Vec<String>,
    index: i64,
    registers: HashMap<String, i64>,
    last_snd: Option<i64>,
    last_rcv: Option<i64>,
}

impl Player {
    fn new(input: &str) -> Self {
        Self {
            prog: input.lines().map(|s| s.to_owned()).collect(),
            index: 0,
            registers: HashMap::new(),
            last_snd: None,
            last_rcv: None,
        }
    }

    fn val(&self, x: &str) -> i64 {
        x.parse::<i64>().unwrap_or(self.get(x))
    }

    fn get(&self, x: &str) -> i64 {
        self.registers.get(&x.to_owned()).unwrap_or(&0).clone()
    }

    fn step(&mut self) {
        if self.index < 0 || self.index >= self.prog.len() as i64 {
            return;
        }
        let line = self.prog[self.index as usize].clone();
        let mut words = line.split_whitespace().map(|w| w.trim());
        let op = words.next().unwrap();
        let x_str = words.next().unwrap();
        let y_str = words.next().unwrap_or("0");
        match op {
            "snd" => self.op_snd(x_str),
            "rcv" => self.op_rcv(x_str),
            // owned ops
            "set" => self.op_set(x_str, y_str),
            "add" => self.op_add(x_str, y_str),
            "mul" => self.op_mul(x_str, y_str),
            "mod" => self.op_mod(x_str, y_str),
            "jgz" => self.op_jgz(x_str, y_str),
            _ => panic!("Invalid operation `{}`", op),
        };
        self.index += 1;
    }

    fn op_snd(&mut self, x_str: &str) {
        self.last_snd = Some(x_str.parse().unwrap_or(self.val(x_str)));
    }

    fn op_rcv(&mut self, x_str: &str) {
        let x = self.get(x_str);
        if x != 0 {
            self.last_rcv = self.last_snd.clone();
        }
    }

    fn op_set(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        self.registers.insert(x, y);
    }

    fn op_add(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers.insert(x, r + y);
    }

    fn op_mul(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers.insert(x, r * y);
    }

    fn op_mod(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers.insert(x, r % y);
    }

    fn op_jgz(&mut self, x_str: &str, y_str: &str) {
        let x: i64 = self.val(x_str);
        if x > 0 {
            let y: i64 = y_str.parse().unwrap_or(0);
            self.index += y - 1;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2\n";
        assert_eq!(part_1(input), 4);
    }
}
