extern crate aoc2017;
use aoc2017::read_fixture;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let pairs = parse_input(&input);
    let mut cost = 0;
    for &(depth, range) in pairs.iter() {
        if depth % (2 * (range - 1)) == 0 {
            cost += depth * range;
        }
    }
    cost
}

fn part_2(input: &str) -> usize {
    let pairs = parse_input(&input);
    let mut offset = 0;
    'outer: loop {
        for &(depth, range) in pairs.iter() {
            if (offset + depth) % (2 * (range - 1)) == 0 {
                offset += 1;
                continue 'outer;
            }
        }
        break;
    }
    offset
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input.lines().map(|line| {
        let mut pair_iter = line.split(": ").map(|w| w.parse().unwrap());
        (pair_iter.next().unwrap(), pair_iter.next().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part_1(&input), 24);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part_2(&input), 10);
    }
}
