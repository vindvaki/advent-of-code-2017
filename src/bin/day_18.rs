extern crate aoc2017;
use aoc2017::read_fixture;

use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(input.as_str()));
    println!("part_2: {}", part_2(input.as_str()));
}

fn part_1(input: &str) -> i64 {
    let mut player = PlayerPart1::new(input);
    while player.last_rcv.is_none() || player.last_rcv == Some(0) {
        player.step();
    }
    player.last_rcv.unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut player_0 = PlayerPart2::new(input);
    let mut player_1 = PlayerPart2::new(input);
    player_0.registers.insert("p".to_owned(), 0);
    player_1.registers.insert("p".to_owned(), 1);
    let mut index_0 = -1;
    let mut index_1 = -1;
    let mut snd_count_1 = 0;
    while player_0.index() != &index_0 || player_1.index() != &index_1 {
        while !player_0.outbox.is_empty() {
            player_1.inbox.push_back(player_0.outbox.pop_front().unwrap());
        }
        while !player_1.outbox.is_empty() {
            player_0.inbox.push_back(player_1.outbox.pop_front().unwrap());
            snd_count_1 += 1;
        }
        index_0 = player_0.index().clone();
        index_1 = player_1.index().clone();
        player_0.step();
        player_1.step();
    }
    snd_count_1
}

struct PlayerPart1 {
    prog: Vec<String>,
    index: i64,
    registers: HashMap<String, i64>,
    last_snd: Option<i64>,
    last_rcv: Option<i64>,
}

impl PlayerPart1 {
    fn new(input: &str) -> Self {
        Self {
            prog: input.lines().map(|s| s.to_owned()).collect(),
            index: 0,
            registers: HashMap::new(),
            last_snd: None,
            last_rcv: None,
        }
    }
}

impl Player for PlayerPart1 {
    fn prog(&self) -> &Vec<String> { &self.prog }
    fn index(&self) -> &i64 { &self.index }
    fn index_mut(&mut self) -> &mut i64 { &mut self.index }
    fn registers(&self) -> &HashMap<String, i64> { &self.registers }
    fn registers_mut(&mut self) -> &mut HashMap<String, i64> { &mut self.registers }

    fn op_snd(&mut self, x_str: &str) {
        self.last_snd = Some(x_str.parse().unwrap_or(self.val(x_str)));
    }

    fn op_rcv(&mut self, x_str: &str) {
        let x = self.get(x_str);
        if x != 0 {
            self.last_rcv = self.last_snd.clone();
        }
    }
}

struct PlayerPart2 {
    prog: Vec<String>,
    index: i64,
    registers: HashMap<String, i64>,
    inbox: VecDeque<i64>,
    outbox: VecDeque<i64>,
}

impl PlayerPart2 {
    fn new(input: &str) -> Self {
        Self {
            prog: input.lines().map(|s| s.to_owned()).collect(),
            index: 0,
            registers: HashMap::new(),
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
        }
    }
}

impl Player for PlayerPart2 {
    fn prog(&self) -> &Vec<String> { &self.prog }
    fn index(&self) -> &i64 { &self.index }
    fn index_mut(&mut self) -> &mut i64 { &mut self.index }
    fn registers(&self) -> &HashMap<String, i64> { &self.registers }
    fn registers_mut(&mut self) -> &mut HashMap<String, i64> { &mut self.registers }

    fn op_snd(&mut self, x_str: &str) {
        let x = self.val(x_str);
        self.outbox.push_back(x);
    }

    fn op_rcv(&mut self, x_str: &str) {
        let x = x_str.to_owned();
        if let Some(y) = self.inbox.pop_front() {
            self.registers_mut().insert(x, y);
        } else {
            self.index -= 1; // wait until next round
        }
    }
}

trait Player {
    fn prog(&self) -> &Vec<String>;
    fn index(&self) -> &i64;
    fn index_mut(&mut self) -> &mut i64;
    fn registers(&self) -> &HashMap<String, i64>;
    fn registers_mut(&mut self) -> &mut HashMap<String, i64>;

    fn val(&self, x: &str) -> i64 {
        x.parse::<i64>().unwrap_or(self.get(x))
    }

    fn get(&self, x: &str) -> i64 {
        self.registers().get(&x.to_owned()).unwrap_or(&0).clone()
    }

    fn step(&mut self) {
        let index = self.index().clone();
        if index < 0 || index >= self.prog().len() as i64 {
            return
        }
        let index = self.index().clone();
        let line = self.prog()[index as usize].clone();
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
        *self.index_mut() += 1;
    }

    fn op_snd(&mut self, x_str: &str);
    fn op_rcv(&mut self, x_str: &str);

    fn op_set(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        self.registers_mut().insert(x, y);
    }

    fn op_add(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers_mut().insert(x, r + y);
    }

    fn op_mul(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers_mut().insert(x, r * y);
    }

    fn op_mod(&mut self, x_str: &str, y_str: &str) {
        let x: String = x_str.to_owned();
        let y: i64 = self.val(y_str);
        let r = self.get(x_str);
        self.registers_mut().insert(x, r % y);
    }

    fn op_jgz(&mut self, x_str: &str, y_str: &str) {
        let x: i64 = self.val(x_str);
        if x > 0 {
            let y: i64 = self.val(y_str);
            *self.index_mut() += y - 1;
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

    #[test]
    fn test_part_2() {
        use part_2;
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(part_2(input), 3);
    }
}
