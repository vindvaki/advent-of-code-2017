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

fn knot_hash_1(
    lengths: &Vec<u8>,
    list: &mut Vec<u8>,
    pos: &mut usize,
    skip: &mut usize,
) {
    let list_len = list.len();
    for len in lengths.iter().map(|&l| l as usize) {
        let begin = *pos;
        let end = (*pos + len) - 1;
        for i in 0..len / 2 {
            list.swap((begin + i) % list_len, (end - i) % list_len);
        }
        *pos = (*pos + len + *skip) % list_len;
        *skip += 1;
    }
}

fn knot_hash(lengths_in: &Vec<u8>) -> String {
    let mut lengths = lengths_in.clone();
    for &i in [17, 31, 73, 47, 23].iter() {
        lengths.push(i as u8);
    }
    let mut list: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        knot_hash_1(&lengths, &mut list, &mut pos, &mut skip);
    }
    let dense_hash_iter = (0..16).map(|block| {
        let begin = block * 16;
        let end = begin + 16;
        list[begin + 1..end].iter().fold(list[begin], |acc, &x| acc ^ x)
    });
    dense_hash_iter.map(|val| format!("{:02x}", val)).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_knot_hash_1() {
        use knot_hash_1;
        let input: Vec<u8> = vec![3, 4, 1, 5];
        let mut list: Vec<u8> = (0..5).collect();
        knot_hash_1(&input, &mut list, &mut 0, &mut 0);
        assert_eq!(list, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn test_knot_hash_empty_vec() {
        use knot_hash;
        assert_eq!(knot_hash(&vec![]), "a2582a3a0e66e6e86e3812dcb672a272".to_owned());
    }

    #[test]
    fn test_knot_hash_aoc_2017() {
        use knot_hash;
        let s: Vec<u8> = "AoC 2017".as_bytes().to_vec();
        assert_eq!(knot_hash(&s), "33efeb34ea91902bb2f59c9920caa6cd".to_owned());
    }
}
