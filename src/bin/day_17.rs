fn main() {
    let step = 376;
    println!("part_1: {}", part_1(step, 2017));
    println!("part_2: {}", part_2(step, 50_000_000));
}

fn part_1(step: usize, count: usize) -> usize {
    use std::collections::VecDeque;
    let mut buf: VecDeque<usize> = VecDeque::new();
    buf.push_back(0);
    let mut pos = 0;
    for i in 1..(count + 1) {
        pos = (pos + step) % buf.len() + 1;
        buf.insert(pos, i);
    }
    buf[(pos + 1) % buf.len()].clone()
}

fn part_2(step: usize, count: usize) -> usize {
    let mut origin = 0;
    let mut pos = 0;
    let mut val = 0;
    for i in 1..(count + 1) {
        pos = (pos + step) % i + 1;
        if pos == (origin + 1) {
            val = i;
        }
        if pos <= origin {
            origin += 1
        }
    }
    val
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1(3, 0), 0, "input = 0");
        assert_eq!(part_1(3, 1), 0, "input = 1");
        assert_eq!(part_1(3, 2), 1, "input = 2");
        assert_eq!(part_1(3, 3), 1, "input = 3");
        assert_eq!(part_1(3, 4), 3, "input = 4");
        assert_eq!(part_1(3, 5), 2, "input = 5");
        assert_eq!(part_1(3, 6), 1, "input = 6");
        assert_eq!(part_1(3, 7), 2, "input = 7");
        assert_eq!(part_1(3, 8), 6, "input = 8");
        assert_eq!(part_1(3, 9), 5, "input = 9");
        assert_eq!(part_1(3, 2017), 638, "2017");
    }

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(part_2(3, 0), 0, "input = 0");
        assert_eq!(part_2(3, 1), 1, "input = 1");
        assert_eq!(part_2(3, 2), 2, "input = 2");
        assert_eq!(part_2(3, 3), 2, "input = 3");
        assert_eq!(part_2(3, 4), 2, "input = 4");
        assert_eq!(part_2(3, 5), 5, "input = 5");
        assert_eq!(part_2(3, 6), 5, "input = 6");
        assert_eq!(part_2(3, 7), 5, "input = 7");
        assert_eq!(part_2(3, 8), 5, "input = 8");
        assert_eq!(part_2(3, 9), 9, "input = 9");
    }
}
