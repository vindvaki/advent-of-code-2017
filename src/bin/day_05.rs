extern crate aoc2017;

use aoc2017::read_fixture;

fn main() {
    let data = read_fixture();
    let maze: Vec<i32> = data
        .split_whitespace()
        .map(|x| str::parse(x).expect("unable to parse number"))
        .collect();
    println!("part_1: {}", part_1(&maze));
    println!("part_2: {}", part_2(&maze));
}

fn part_1(maze_in: &Vec<i32>) -> i32 {
    let mut maze = maze_in.clone();
    let mut index: i32 = 0;
    let mut steps: i32 = 0;
    while index >= 0 && index < maze.len() as i32 {
        let next = index + maze[index as usize];
        steps += 1;
        maze[index as usize] += 1;
        index = next;
    }
    steps
}

fn part_2(maze_in: &Vec<i32>) -> i32 {
    let mut maze = maze_in.clone();
    let mut index: i32 = 0;
    let mut steps: i32 = 0;
    while index >= 0 && index < maze.len() as i32 {
        let offset = maze[index as usize];
        let next = index + offset;
        if offset >= 3 {
            maze[index as usize] -= 1;
        } else {
            maze[index as usize] += 1;
        }
        steps += 1;
        index = next;
    }
    steps
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let maze = vec![0, 3, 0, 1, -3];
        assert_eq!(part_1(&maze), 5);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let maze = vec![0, 3, 0, 1, -3];
        assert_eq!(part_2(&maze), 10);
    }
}

