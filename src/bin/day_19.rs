extern crate aoc2017;
use aoc2017::read_fixture;

type Board = Vec<Vec<char>>;

fn main() {
    let input = read_fixture();
    let board: Board = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}
