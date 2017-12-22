extern crate aoc2017;

use aoc2017::read_fixture;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    let input = read_fixture();
    for line in input.lines() {
        for point_s in line.split(", ") {
            let point: Point3 = point_s.parse().expect("unable to parse point");
            println!("{:?}", point);
        }
    }
}

#[derive(Debug)]
struct Point3(i64, i64, i64);

#[derive(Debug)]
enum ParsePoint3Error {
    ParseIntError(ParseIntError),
    WrongCoordinateCount(usize),
}

impl From<ParseIntError> for ParsePoint3Error {
    fn from(error: ParseIntError) -> Self {
        ParsePoint3Error::ParseIntError(error)
    }
}

impl fmt::Display for ParsePoint3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParsePoint3Error {
    fn description(&self) -> &str {
        "Could not parse Point3"
    }
}

impl std::str::FromStr for Point3 {
    type Err = ParsePoint3Error;

    fn from_str(point3_str: &str) -> Result<Self, Self::Err> {
        let outer_delims: &[_] = &['p', 'a', 'v', '=', '<', '>', ' '];
        let coords_res: Result<Vec<i64>, ParseIntError> = point3_str
            .trim_matches(outer_delims)
            .split(",")
            .map(|i64_str| i64_str.parse())
            .collect();
        let coords = coords_res?;
        if coords.len() != 3 {
            return Err(ParsePoint3Error::WrongCoordinateCount(coords.len()))
        }
        Ok(Point3(coords[0], coords[1], coords[2]))
    }
}
