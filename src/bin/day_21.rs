extern crate aoc2017;
use aoc2017::read_fixture;

use std::collections::HashMap;

const PATTERN_S: &'static str = ".#.
..#
###";

fn main() {
    let rules_s = read_fixture();
    println!("part_1: {}", part_1(PATTERN_S, &rules_s, 5));
    println!("part_1: {}", part_1(PATTERN_S, &rules_s, 18));
}

fn part_1(pattern_s: &str, rules_s: &str, count: usize) -> usize {
    let rule_map = parse_rules(rules_s);
    let mut grid = parse_pattern(pattern_s);
    for _ in 0..count {
        // current grid dim
        let m = (grid.len() as f64).sqrt() as usize;
        // input subpattern, output subpattern
        let (a, b) = if m % 2 == 0 { (2, 3) } else { (3, 4) };
        // next grid dim
        let n = (m / a) * b;

        // allocate next grid
        let mut next_grid = Vec::new();
        for _ in 0..n { for _ in 0..n { next_grid.push(false) } }

        // write subdivision
        for x in 0..m/a {
            for y in 0..m/a {
                let mut subpattern = Vec::new();
                for xi in 0..a {
                    let i = x*a + xi;
                    for yj in 0..a {
                        let j = y*a + yj;
                        subpattern.push(grid[j + i*m]);
                    }
                }
                let transformed = &rule_map[&subpattern];
                for xi in 0..b {
                    let i = x*b + xi;
                    for yj in 0..b {
                        let j = y*b + yj;
                        next_grid[j + i*n] = transformed[yj + xi*b];
                    }
                }
            }
        }
        grid = next_grid;
    }
    grid.iter().map(|&b| b as usize).sum()
}

fn pattern_to_s(pattern: &Pattern) -> String {
    let n = (pattern.len() as f64).sqrt() as usize;
    let mut out = String::new();
    for i in 0..n {
        for j in 0..n {
            out.push(if pattern[j + i*n] { '#' } else { '.' });
        }
        if i+1 != n {
            out.push('\n');
        }
    }
    out
}

type Pattern = Vec<bool>;
type RuleMap = HashMap<Pattern, Pattern>;

fn parse_rules(input: &str) -> RuleMap {
    let mut rule_map = HashMap::new();

    for line in input.lines() {
        let (pattern, out) = parse_rule(line);
        for p in permutations(&pattern) {
            if !rule_map.contains_key(&p) {
                rule_map.insert(p, out.clone());
            }
        }
    }

    rule_map
}

fn parse_rule(rule: &str) -> (Pattern, Pattern) {
    let mut rule_split = rule.split(" => ");
    let pattern_s = rule_split.next().unwrap();
    let out_s = rule_split.next().unwrap();
    let pattern = parse_pattern(&pattern_s);
    let out = parse_pattern(&out_s);
    (pattern, out)
}

fn parse_pattern(pattern_s: &str) -> Pattern {
    pattern_s.chars().filter_map(|c| {
        match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        }
    }).collect()
}

fn permutations(pattern: &Pattern) -> Vec<Pattern> {
    let mut result = Vec::new();
    let mut current = pattern.clone();
    for _ in 0..4 {
        result.push(current.clone());
        result.push(flip_lr(&current));
        current = rot_90(&current);
    }
    result
}

fn rot_90<T: Clone>(x: &Vec<T>) -> Vec<T> {
    let t = transpose(x);
    flip_lr(&t)
}

fn transpose<T: Clone>(x: &Vec<T>) -> Vec<T> {
    let n = (x.len() as f64).sqrt() as usize;
    let mut y = Vec::new();
    for i in 0..n {
        for j in 0..n {
            y.push(x[i + j*n].clone());
        }
    }
    y
}

fn flip_lr<T: Clone>(x: &Vec<T>) -> Vec<T> {
    let n = (x.len() as f64).sqrt() as usize;
    let mut y = Vec::new();
    for i in 0..n {
        for j in 0..n {
            y.push(x[(n - j - 1) + i*n].clone());
        }
    }
    y
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        use PATTERN_S;

        let rules_s = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        assert_eq!(part_1(PATTERN_S, rules_s, 2), 12);
    }
}

