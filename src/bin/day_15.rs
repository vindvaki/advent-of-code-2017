const A_MUL: u64 = 16807;
const B_MUL: u64 = 48271;
const MASK: u64 = 0xffff;

fn main() {
    let a0 = 783;
    let b0 = 325;
    println!("part_1: {}", part_1(a0, b0, 40_000_000));
    println!("part_2: {}", part_2(a0, b0, 5_000_000));
}

fn part_1(a0: u64, b0: u64, rounds: usize) -> usize {
    let gen_a = ProdModStep(a0, A_MUL);
    let gen_b = ProdModStep(b0, B_MUL);
    gen_a.zip(gen_b).take(rounds).filter(|&(a, b)| a & MASK == b & MASK).count()
}

fn part_2(a0: u64, b0: u64, rounds: usize) -> usize {
    let gen_a = ProdModStep(a0, A_MUL).filter(|&a| a % 4 == 0);
    let gen_b = ProdModStep(b0, B_MUL).filter(|&b| b % 8 == 0);
    gen_a.zip(gen_b).take(rounds).filter(|&(a, b)| a & MASK == b & MASK).count()
}

struct ProdModStep(u64, u64);

impl Iterator for ProdModStep {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = (self.0 * self.1) % 2147483647;
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1(65, 8921, 5), 1);
    }

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(part_2(65, 8921, 5_000_000), 309);
    }
}
