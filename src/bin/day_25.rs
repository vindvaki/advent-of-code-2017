use std::collections::HashMap;

fn main() {
    let steps = 12134527;
    println!("part_1: {}", part_1(steps));
}

fn part_1(steps: usize) -> usize {
    let mut tm = TuringMachine::new();
    for _ in 0..steps {
        tm.step();
    }
    tm.checksum()
}

#[derive(Debug)]
struct TuringMachine {
    tape: HashMap<i64, bool>,
    cursor: i64,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum State { A, B, C, D, E, F }

impl TuringMachine {
    fn new() -> Self {
        Self {
            tape: HashMap::new(),
            cursor: 0,
            state: State::A,
        }
    }
    fn step(&mut self) {
        let value = *self.tape.get(&self.cursor).unwrap_or(&false);
        let r = 1;
        let l = -1;
        use State::*;
        let (write, direction, next) = match self.state {
            A => if !value {
                (1, r, B)
            } else {
                (0, l, C)
            },
            B => if !value {
                (1, l, A)
            } else {
                (1, r, C)
            },
            C => if !value {
                (1, r, A)
            } else {
                (0, l, D)
            },
            D => if !value {
                (1, l, E)
            } else {
                (1, l, C)
            },
            E => if !value {
                (1, r, F)
            } else {
                (1, r, A)
            },
            F => if !value {
                (1, r, A)
            } else {
                (1, r, E)
            },
        };
        self.tape.insert(self.cursor, write == 1);
        self.cursor += direction;
        self.state = next;
    }

    fn checksum(&self) -> usize {
        self.tape.values().map(|&v| v as usize).sum()
    }
}
