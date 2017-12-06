extern crate aoc2017;

use aoc2017::read_fixture;

fn main() {
    let data = read_fixture();
    let blocks: Vec<i32> = data
        .split_whitespace()
        .map(|x| str::parse(x).expect("unable to parse number"))
        .collect();
    println!("part_1: {}", part_1(&blocks));
    println!("part_2: {}", part_2(&blocks));
}

fn part_1(blocks: &Vec<i32>) -> i32 {
    let mut seen = std::collections::HashSet::new();
    seen.insert(blocks.clone());
    let mut iter = blocks.clone();
    let mut count = 0;
    loop {
        count += 1;
        spread_max(&mut iter);
        if seen.contains(&iter) {
            break;
        }
        seen.insert(iter.clone());
    }
    count
}

fn part_2(blocks: &Vec<i32>) -> i32 {
    let mut seen = std::collections::HashMap::new();
    seen.insert(blocks.clone(), 0);
    let mut iter = blocks.clone();
    let mut count = 0;
    let result;
    loop {
        count += 1;
        spread_max(&mut iter);
        if let Some(other_count) = seen.get(&iter) {
            result = count - other_count;
            break;
        }
        seen.insert(iter.clone(), count);
    }
    result
}

fn spread_max(blocks: &mut Vec<i32>) {
    let (i, max) = {
        let iter = blocks.iter();
        match iter.enumerate().max_by_key(|&(i, &val)| (val, -(i as i32))) {
            Some((i, &val)) => (i, val),
            None => return,
        }
    };
    blocks[i] = 0;
    let n = blocks.len() as i32;
    let q = max / n;
    let mut r = max % n;
    let j0 = i as i32 + 1;
    for k in 0..n {
        let j = (j0 + k) % n;
        blocks[j as usize] += q;
        if r > 0 {
            blocks[j as usize] += 1;
            r -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2() {
        use part_2;
        let blocks = vec![0, 2, 7, 0];
        assert_eq!(part_2(&blocks), 4);
    }

    #[test]
    fn test_part_1() {
        use part_1;
        let blocks = vec![0, 2, 7, 0];
        assert_eq!(part_1(&blocks), 5);
    }

    #[test]
    fn test_spread_max() {
        use spread_max;
        let mut blocks = vec![0, 2, 7, 0];
        spread_max(&mut blocks);
        assert_eq!(blocks, vec![2, 4, 1, 2]);
    }
}

