extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::HashMap;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
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
    fn from_str(s: &str) -> Self {
        let (name, rest) = s.split_at(3);
        let mut values = rest.split_whitespace();
        let x = values.next().unwrap().to_owned();
        let y = values.next().unwrap().to_owned();
        Self { name: name.to_owned(), x, y }
    }
}

impl Program {
    fn from_str(s: &str) -> Self {
        Self {
            pos: 0,
            instructions: s.lines().map(|s| Instruction::from_str(s)).collect(),
            registers: HashMap::new(),
        }
    }

    fn get(&self, s: &String) -> i64 {
        self.registers.get(s).unwrap_or(&0).clone()
    }

    fn val(&self, s: &String) -> i64 {
        match s.parse::<i64>() {
            Ok(r) => r,
            _ => self.get(s),
        }
    }

    fn step(&mut self) {
        if self.pos < 0 || self.pos >= self.instructions.len() as i64 {
            return;
        }
        let Instruction { ref name, ref x, ref y } = self.instructions[self.pos as usize];
        let x_val = self.val(&x);
        let y_val = self.val(&y);
        match name.as_str() {
            "set" => { self.registers.insert(x.clone(), y_val); },
            "sub" => { self.registers.insert(x.clone(), x_val - y_val); },
            "mul" => { self.registers.insert(x.clone(), x_val * y_val); },
            "jnz" => if x_val != 0 { self.pos += y_val - 1; },
            &_ => {},
        }
        self.pos += 1;
    }
}
