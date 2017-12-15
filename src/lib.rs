use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn read_fixture() -> String {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("fixture path must be provided");
    let mut file = File::open(path).expect("unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("unable to read file");
    data
}

pub fn knot_hash_1(
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

pub fn knot_hash(lengths_in: &Vec<u8>) -> String {
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

