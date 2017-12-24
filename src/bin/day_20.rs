extern crate aoc2017;

use aoc2017::read_fixture;
use std::fmt;
use std::num::ParseIntError;
use std::fmt::{Debug, Display};
use std::error::Error;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = read_fixture();
    println!("{}", input.lines().count());
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let particles = parse_particles(input);
    (0..particles.len()).min_by_key(|&i| particles[i].a.norm_1()).unwrap()
}

fn part_2(input: &str) -> usize {
    let particles = parse_particles(input);

    // gather collisions
    let mut collisions = HashMap::new();
    for i in 0..particles.len() {
        for j in 0..i {
            let t = particles[i].collision_time(&particles[j]);
            if !collisions.contains_key(&t) {
                collisions.insert(t, HashSet::new());
            }
            if let Some(cs) = collisions.get_mut(&t) {
                cs.insert(i);
                cs.insert(j);
            }
        }
    }

    // eliminate in collision order
    let mut eliminated = HashSet::new();
    for (t, collisions_at_t) in collisions.iter() {
        let to_eliminate: Vec<usize> = collisions_at_t
            .difference(&eliminated)
            .map(|i| i.clone())
            .collect();
        if to_eliminate.len() > 1 {
            for &i in to_eliminate.iter() {
                eliminated.insert(i);
            }
        }
    }
    particles.len() - eliminated.len()
}

fn parse_particles(input: &str) -> Vec<Particle> {
    input.lines()
        .map(|line| line.parse().expect("Unable to parse particle"))
        .collect()
}

#[derive(Debug)]
struct Particle {
    p: Point3,
    v: Point3,
    a: Point3,
}

fn is_nonnegative_integer(x: f64) -> bool {
    x >= 0.0 && (x - x.round()).abs() <= std::f64::EPSILON
}

// a*x*x + b*x + c
fn nonnegative_integer_solutions(a: f64, b: f64, c: f64) -> HashSet<i64> {
    let mut solutions = HashSet::new();
    if a <= std::f64::EPSILON {
        if b <= std::f64::EPSILON {
            if c <= std::f64::EPSILON {
                solutions.insert(0);
            }
            return solutions;
        }
        let x = c / b;
        if is_nonnegative_integer(x) { solutions.insert(x as i64); }
        return solutions;
    }
    let ss = b*b - 4.0*a*c;
    if ss < 0.0 {
        return solutions;
    }
    let s = ss.sqrt();
    let l = (-b + s) / (2.0 * a);
    let r = (-b - s) / (2.0 * a);
    if is_nonnegative_integer(l) { solutions.insert(l as i64); }
    if is_nonnegative_integer(r) { solutions.insert(r as i64); }
    solutions
}


impl Particle {
    fn point(&self, t: i64) {
        let tt = t*t;
        Point3(
            self.p.0 + self.v.0 * t + self.a.0 * tt,
            self.p.1 + self.v.1 * t + self.a.1 * tt,
            self.p.2 + self.v.2 * t + self.a.2 * tt,
        );
    }

    fn collision_time(&self, other: &Particle) -> Option<i64> {
        let mut s0 = nonnegative_integer_solutions(
            (self.p.0 - other.p.0) as f64,
            (self.v.0 - other.v.0) as f64,
            (self.a.0 - other.a.0) as f64,
        );
        let mut s1 = nonnegative_integer_solutions(
            (self.p.1 - other.p.1) as f64,
            (self.v.1 - other.v.1) as f64,
            (self.a.1 - other.a.1) as f64,
        );
        s0 = s0.intersection(&s1).map(|&i| i.clone()).collect();
        s1 = nonnegative_integer_solutions(
            (self.p.2 - other.p.2) as f64,
            (self.v.2 - other.v.2) as f64,
            (self.a.2 - other.a.2) as f64,
        );
        s0 = s0.intersection(&s1).map(|&i| i.clone()).collect();
        match s0.iter().next() {
            Some(&i) => Some(i),
            None => None,
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
