extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::HashSet;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut components: Vec<(usize, usize)>= vec![(0,0)];
    for line in input.lines() {
        components.push(parse_component(line));
    }
    let mut combination_vec = vec![0];
    let mut combination_set = HashSet::new();
    combination_set.insert(0);
    traverse_1(components.len(), &mut combination_vec, &mut combination_set,  &mut components)
}

fn part_2(input: &str) -> usize {
    let mut components: Vec<(usize, usize)>= vec![(0,0)];
    for line in input.lines() {
        components.push(parse_component(line));
    }
    let mut combination_vec = vec![0];
    let mut combination_set = HashSet::new();
    combination_set.insert(0);
    let mut max = (0,0);
    traverse_2(
        components.len(),
        0,
        &mut max,
        &mut combination_vec,
        &mut combination_set,
        &mut components,
    );
    max.1
}


fn traverse_1(
    n: usize,
    combination_vec: &mut Vec<usize>,
    combination_set: &mut HashSet<usize>,
    components: &mut Vec<(usize, usize)>,
) -> usize
{
    let mut result = 0;

    let i = combination_vec[combination_vec.len() - 1];
    for j in 0..n {
        if combination_set.contains(&j) {
            continue;
        }
        if components[i].1 == components[j].1 {
            let tmp = components[j].0;
            components[j].0 = components[j].1;
            components[j].1 = tmp;
        }
        if components[i].1 != components[j].0 {
            continue;
        }
        let x = components[j].0 + components[j].1;
        combination_vec.push(j);
        combination_set.insert(j);
        let s = x + traverse_1(n, combination_vec, combination_set, components);
        if s > result {
            result = s;
        }
        combination_vec.pop();
        combination_set.remove(&j);
    }
    result
}

fn traverse_2(
    n: usize,
    sum: usize,
    max: &mut (usize, usize),
    combination_vec: &mut Vec<usize>,
    combination_set: &mut HashSet<usize>,
    components: &mut Vec<(usize, usize)>,
)
{
    *max = (*max).max((combination_vec.len(), sum));

    let i = combination_vec[combination_vec.len() - 1];
    for j in 0..n {
        if combination_set.contains(&j) {
            continue;
        }
        if components[i].1 == components[j].1 {
            let tmp = components[j].0;
            components[j].0 = components[j].1;
            components[j].1 = tmp;
        }
        if components[i].1 != components[j].0 {
            continue;
        }
        let x = components[j].0 + components[j].1;
        combination_vec.push(j);
        combination_set.insert(j);
        traverse_2(n, sum + x, max, combination_vec, combination_set, components);
        combination_vec.pop();
        combination_set.remove(&j);
    }
}


fn parse_component(input: &str) -> (usize, usize) {
    let mut iter = input
        .split('/')
        .map(|s| s.parse::<usize>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    (a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(part_1(input), 31);
    }
}
