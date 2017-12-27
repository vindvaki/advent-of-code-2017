extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::HashMap;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input, 10_000));
    println!("part_2: {}", part_2(&input, 10_000_000));
}

fn part_1(input: &str, count: usize) -> usize {
    let mut grid = Grid1::from_str(input);
    (0..count).map(|_| grid.step() as usize).sum()
}

fn part_2(input: &str, count: usize) -> usize {
    let mut grid = Grid2::from_str(input);
    (0..count).map(|_| (grid.step() == InfectionState::Infected)as usize).sum()
}

struct Grid1 {
    pos: (i64, i64),
    dir: (i64, i64),
    infection_map: HashMap<(i64, i64), bool>,
}

impl Grid1 {
    fn from_str(input: &str) -> Self {
        let values: Vec<Vec<bool>> = input.lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let m = values.len() as i64;
        let n = values[0].len() as i64;
        let (i0, j0) = (m/2, n/2);
        let mut infection_map = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                infection_map.insert(((m - i - 1) - i0, j - j0), values[i as usize][j as usize]);
            }
        }
        Self {
            pos: (0, 0), // middle
            dir: (1, 0), // up
            infection_map,
        }
    }

    fn step(&mut self) -> bool {
        let infected = self.infection_map.get(&self.pos).unwrap_or(&false).clone();
        // turn
        self.dir = if infected {
            // right
            match self.dir {
                (0, 1)  => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                _ => panic!("invalid direction"),
            }
        } else {
            // left
            match self.dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("invalid direction"),
            }
        };
        // flip infection
        self.infection_map.insert(self.pos.clone(), !infected);
        // move
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
        !infected
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InfectionState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl InfectionState {
    fn from_char(c: char) -> Self {
        use InfectionState::*;
        // input does not contain flagged or weakened states
        match c {
            '#' => Infected,
            _ => Clean,
        }
    }

    fn next(&self) -> Self {
        use InfectionState::*;
        match self {
            &Clean => Weakened,
            &Weakened => Infected,
            &Infected => Flagged,
            &Flagged => Clean,
        }
    }
}

struct Grid2 {
    pos: (i64, i64),
    dir: (i64, i64),
    infection_map: HashMap<(i64, i64), InfectionState>,
}

impl Grid2 {
    fn from_str(input: &str) -> Self {
        let values: Vec<Vec<InfectionState>> = input.lines()
            .map(|line| line.chars().map(|c| InfectionState::from_char(c)).collect())
            .collect();

        let m = values.len() as i64;
        let n = values[0].len() as i64;
        let (i0, j0) = (m/2, n/2);
        let mut infection_map = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                infection_map.insert(((m - i - 1) - i0, j - j0), values[i as usize][j as usize].clone());
            }
        }
        Self {
            pos: (0, 0), // middle
            dir: (1, 0), // up
            infection_map,
        }
    }

    fn step(&mut self) -> InfectionState {
        use InfectionState::*;
        let state = self.infection_map.get(&self.pos).unwrap_or(&Clean).clone();
        // turn
        self.dir = match state {
            Clean => match self.dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("invalid direction"),
            },
            Weakened => self.dir,
            Infected => match self.dir {
                (0, 1)  => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                _ => panic!("invalid direction"),
            },
            Flagged => (-self.dir.0, -self.dir.1),
        };
        // flip infection
        self.infection_map.insert(self.pos.clone(), state.next().clone());
        // move
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
        state.next()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input = "..#
#..
...";
        assert_eq!(part_1(input, 7), 5);
        assert_eq!(part_1(input, 70), 41);
        assert_eq!(part_1(input, 10_000), 5587);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let input = "..#
#..
...";
        assert_eq!(part_2(input, 100), 26);
        assert_eq!(part_2(input, 10_000_000), 2_511_944);
    }
}

