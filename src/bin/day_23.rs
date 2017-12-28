extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::HashMap;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(1));
}

fn part_1(input: &str) -> usize {
    let mut program = Program::from_str(input);
    let mut result = 0;
    while program.pos >= 0 && (program.pos as usize) < program.instructions.len() {
        if program.instructions[program.pos as usize].name == "mul" {
            result += 1;
        }
        program.step();
    }
    result
}

fn part_2(a: usize) -> usize {
    let mut b: usize = 99;
    let mut c: usize = 99;
    let mut h: usize = 0;

    if a != 0 {
        b = 109_900;
        c = b + 17_000;
    }
    while b <= c {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += 17;
    }
    h
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    pos: i64,
    instructions: Vec<Instruction>,
    registers: HashMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    name: String,
    x: String,
    y: String,
}

impl Instruction {
    fn from_str(s: &str) -> Option<Self> {
        let (name, rest) = s.split_at(3);
        let mut values = rest.split_whitespace();

        let x = match values.next() {
            Some(v) => v.to_owned(),
            None => return None,
        };

        let y = match values.next() {
            Some(v) => v.to_owned(),
            None => return None,
        };

        Some(Self { name: name.to_owned(), x, y })
    }
}

impl Program {
    fn from_str(s: &str) -> Self {
        let instructions = s.trim().lines()
            .map(|l| l.trim())
            .filter_map(|l| Instruction::from_str(l))
            .collect();

        Self {
            pos: 0,
            instructions: instructions,
            registers: HashMap::new(),
        }
    }

    fn get(&self, s: &str) -> i64 {
        self.registers.get(&s.to_owned()).unwrap_or(&0).clone()
    }

    fn val(&self, s: &str) -> i64 {
        match s.parse::<i64>() {
            Ok(r) => r,
            _ => self.get(s),
        }
    }

    fn step(&mut self) {
        if self.pos < 0 || self.pos >= self.instructions.len() as i64 {
            return;
        }
        let Instruction { name, x, y } = self.instructions[self.pos as usize].clone();
        match name.as_str() {
            "set" => self.set(&x, &y),
            "sub" => self.sub(&x, &y),
            "mul" => self.mul(&x, &y),
            "jnz" => self.jnz(&x, &y),
            &_ => {},
        }
        self.pos += 1;
    }

    fn set(&mut self, x: &str, y: &str) {
        let y_val = self.val(y);
        self.registers.insert(x.to_owned(), y_val);
    }

    fn sub(&mut self, x: &str, y: &str) {
        let x_val = self.val(x);
        let y_val = self.val(y);
        self.registers.insert(x.to_owned(), x_val - y_val);
    }

    fn mul(&mut self, x: &str, y: &str) {
        let x_val = self.val(x);
        let y_val = self.val(y);
        self.registers.insert(x.to_owned(), x_val * y_val);
    }

    fn jnz(&mut self, x: &str, y: &str) {
        let x_val = self.val(x);
        if x_val != 0 {
            let y_val = self.val(y);
            self.pos += y_val - 1;
        }
    }
}
