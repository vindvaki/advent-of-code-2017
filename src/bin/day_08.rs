extern crate aoc2017;

use aoc2017::read_fixture;

use std::collections::HashMap;

fn main() {
    let data = read_fixture();
    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(data: &str) -> i32 {
    let mut registers = HashMap::new();
    for line in data.lines() {
        apply_instruction(line, &mut registers);
    }
    registers.values().max().unwrap().to_owned()
}

fn part_2(data: &str) -> i32 {
    let mut registers = HashMap::new();
    data.lines().map(|line| apply_instruction(line, &mut registers)).max().unwrap()
}

fn apply_instruction(instruction: &str, registers: &mut HashMap<String, i32>) -> i32 {
    let mut iter = instruction.split_whitespace();
    let label = iter.next().unwrap().to_owned();
    let add_op = iter.next().unwrap();
    let add_val: i32 = iter.next().unwrap().parse().unwrap();
    iter.next(); // if
    let cmp_label = iter.next().unwrap().to_owned();
    let cmp_op = iter.next().unwrap();
    let cmp_val: i32 = iter.next().unwrap().parse().unwrap();
    let cmp_r: i32 = *registers.get(&cmp_label).unwrap_or(&0);
    let cmp_result = match cmp_op {
        "!=" => cmp_r != cmp_val,
        "==" => cmp_r == cmp_val,
        ">=" => cmp_r >= cmp_val,
        "<=" => cmp_r <= cmp_val,
        ">" => cmp_r > cmp_val,
        "<" => cmp_r < cmp_val,
        &_ => panic!("invalid comparison"),
    };
    let r = *registers.get(&label).unwrap_or(&0);
    let inc = if cmp_result {
        match add_op {
            "inc" => add_val,
            "dec" => -add_val,
            &_ => panic!("invalid increment"),
        }
    } else {
        0
    };
    let value = r + inc;
    registers.insert(label, value);
    value
}

#[cfg(tests)]
mod tests {
}
