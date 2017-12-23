extern crate aoc2017;

use aoc2017::read_fixture;
use std::fmt;
use std::num::ParseIntError;
use std::fmt::{Debug, Display};
use std::error::Error;
use std::str::FromStr;

fn main() {
    let input = read_fixture();
    for line in input.lines() {
        let particle = line.parse::<Particle>();
        println!("{:?}", particle);
    }
}

#[derive(Debug)]
struct Particle {
    p: Point3,
    v: Point3,
    a: Point3,
}

type ParseParticleError = ParseSplitError<ParsePoint3Error>;

impl FromStr for Particle {
    type Err = ParseParticleError;

    fn from_str(particle_str: &str) -> Result<Self, Self::Err> {
        let points_res: Result<Vec<Point3>, ParsePoint3Error> = particle_str
            .trim()
            .split(", ")
            .map(|point_str| point_str.parse())
            .collect();
        let points = points_res?;
        if points.len() != 3 {
            return Err(ParseSplitError::WrongSize(points.len()))
        }
        Ok(Particle { p: points[0], v: points[1], a: points[2] })
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3(i64, i64, i64);

impl Point3 {
    fn norm_1(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

#[derive(Debug)]
enum ParseSplitError<E: Debug> {
    ParseComponentError(E),
    WrongSize(usize),
}

impl<E: Error> From<E> for ParseSplitError<E> {
    fn from(error: E) -> Self {
        ParseSplitError::ParseComponentError(error)
    }
}

impl<E: Error> Display for ParseSplitError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: Error> Error for ParseSplitError<E> {
    fn description(&self) -> &str {
        "Could not parse split"
    }
}

type ParsePoint3Error = ParseSplitError<ParseIntError>;

impl FromStr for Point3 {
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
            return Err(ParseSplitError::WrongSize(coords.len()))
        }
        Ok(Point3(coords[0], coords[1], coords[2]))
    }
}
