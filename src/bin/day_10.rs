extern crate aoc2017;
use aoc2017::knot_hash;
use aoc2017::knot_hash_1;

fn main() {
    let input_s = "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88";
    let input_nums: Vec<u8> = input_s.split(',').map(|w| w.parse().unwrap()).collect();
    let input_bytes = input_s.as_bytes().to_vec();
    println!("part_1: {}", part_1(&input_nums));
    println!("part_2: {}", part_2(&input_bytes));
}

fn part_1(lengths: &Vec<u8>) -> usize {
    let mut list: Vec<u8> = (0..256).map(|i| i as u8).collect();
    knot_hash_1(lengths, &mut list, &mut 0, &mut 0);
    (list[0] as usize) * (list[1] as usize)
}

fn part_2(bytes: &Vec<u8>) -> String {
    knot_hash(&bytes)
}


#[cfg(test)]
mod tests {
}
