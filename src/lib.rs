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

#[cfg(test)]
mod tests {
}

