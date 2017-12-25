extern crate aoc2017;

use aoc2017::read_fixture;
use std::fmt;
use std::num::ParseIntError;
use std::fmt::{Debug, Display};
use std::error::Error;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let particles: Vec<Particle> = input.lines()
        .map(|line| line.parse().expect("Unable to parse particle"))
        .collect();
    (0..particles.len()).min_by_key(|&i| particles[i].a.norm_1()).unwrap()
}

fn part_2(input: &str) -> usize {
    let mut particles = HashMap::new();
    for line in input.lines() {
        let particle: Particle = line.parse().expect("Unable to parse");
        particles.insert(particle.p, particle);
    }
    for _ in 0..500 {
        let mut next_particles = HashMap::new();
        let mut seen = HashSet::new();
        for (_, particle) in particles.iter() {
            let q = particle.step();
            if seen.contains(&q.p) {
                next_particles.remove(&q.p);
            } else {
                seen.insert(q.p.clone());
                next_particles.insert(q.p.clone(), q);
            }
        }
        particles = next_particles;
    }
    particles.len()
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Particle {
    p: Point3,
    v: Point3,
    a: Point3,
}

impl Particle {
    fn step(&self) -> Particle {
        let v = self.v + self.a;
        Particle {
            p: self.p + v,
            v: v,
            a: self.a,
        }
    }
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point3(i64, i64, i64);

impl Point3 {
    fn norm_1(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Self) -> Point3 {
        Point3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2() {
        use part_2;
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        assert_eq!(part_2(input), 1);
        // p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>"
    }
}
