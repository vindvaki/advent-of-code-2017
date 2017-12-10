extern crate aoc2017;
use aoc2017::read_fixture;

fn main() {
    let data = read_fixture();
    println!("part_1 {}", part_1(&data));
    println!("part_2 {}", part_2(&data));
}

fn part_1(data: &str) -> i64 {
    let mut score = 0;
    let mut level = 1;
    let mut garbage = false;
    let mut cancelled  = false;
    for c in data.chars() {
        if cancelled {
            cancelled = false;
            continue;
        }
        if c == '!' {
            cancelled = true;
            continue;
        }
        if garbage {
            if c == '>' {
                garbage = false;
            }
            continue;
        }
        if c == '<' {
            garbage = true;
            continue;
        }
        if c == '{' {
            level += 1;
            continue;
        }
        if c == '}' {
            level -= 1;
            score += level;
            continue;
        }
    }
    score
}

fn part_2(data: &str) -> i64 {
    let mut garbage_count = 0;
    let mut garbage = false;
    let mut cancelled  = false;
    for c in data.chars() {
        if cancelled {
            cancelled = false;
            continue;
        }
        if c == '!' {
            cancelled = true;
            continue;
        }
        if garbage {
            if c == '>' {
                garbage = false;
            } else {
                garbage_count += 1;
            }
            continue;
        }
        if c == '<' {
            garbage = true;
            continue;
        }
    }
    garbage_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1("{}"), 1);
        assert_eq!(part_1("{{{}}}"), 6);
        assert_eq!(part_1("{{},{}}"), 5);
        assert_eq!(part_1("{{{},{},{{}}}}"), 16);
        assert_eq!(part_1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part_1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part_1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part_1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(part_2("<>"), 0);
        assert_eq!(part_2("<random characters>"), 17);
        assert_eq!(part_2("<<<<>"), 3);
        assert_eq!(part_2("<{!>}>"), 2);
        assert_eq!(part_2("<!!>"), 0);
        assert_eq!(part_2("<!!!>>"), 0);
        assert_eq!(part_2("<{o\"i!a,<{i<a>"), 10);
    }
}
