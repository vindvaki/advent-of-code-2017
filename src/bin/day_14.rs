extern crate aoc2017;
use aoc2017::knot_hash;

fn main() {
    let input = "hxtvlmkl";
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    knot_hash_grid(input).iter()
        .map(|row| row.iter().sum::<usize>())
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut grid = knot_hash_grid(input);
    let mut count = 2;
    for i in 0..128 {
        for j in 0..128 {
            if grid[i as usize][j as usize] == 1 {
                grid_dfs(i, j, count, &mut grid);
                count += 1;
            }
        }
    }
    count - 2
}

fn grid_dfs(i: i32, j: i32, label: usize, grid: &mut Vec<Vec<usize>>) {
    if i < 0 || i >= 128 || j < 0 || j >= 128 {
        return;
    }
    if grid[i as usize][j as usize] != 1 {
        return;
    }
    grid[i as usize][j as usize] = label;
    grid_dfs(i + 1, j, label, grid);
    grid_dfs(i - 1, j, label, grid);
    grid_dfs(i, j + 1, label, grid);
    grid_dfs(i, j - 1, label, grid);
}

fn knot_hash_grid(input: &str) -> Vec<Vec<usize>> {
    let mut grid = Vec::new();
    for i in 0..128 {
        let s = format!("{}-{}", input, i);
        let b: Vec<u8> = s.as_bytes().to_vec();
        let h = knot_hash(&b);
        let mut row = Vec::new();
        for c in h.chars() {
            let bits = format!("{:04b}", c.to_digit(16).unwrap());
            for bit in bits.chars() {
                row.push(bit.to_digit(2).unwrap() as usize);
            }
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(part_2("flqrgnkx"), 1242);
    }
}

