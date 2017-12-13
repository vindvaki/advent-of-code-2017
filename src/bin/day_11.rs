extern crate aoc2017;
use aoc2017::read_fixture;

use std::ops::{Add, AddAssign, Sub, SubAssign};

extern crate num;
use num::Float;

fn main() {
    let data = read_fixture();
    let steps: Vec<&str> = data.split(',').map(|s| s.trim()).collect();
    println!("part_1: {}", part_1(&steps));
    // println!("part_2: {}", part_2(&steps));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2 {
    x: f64,
    y: f64,
}

impl Point2 {
    fn new(x: f64, y: f64) -> Self {
        Point2 { x, y }
    }

    fn dot(self, other: Point2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn distance(self, other: Point2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn norm(self) -> f64 {
        self.distance(Point2::new(0.0, 0.0))
    }

    fn scale(self, s: f64) -> Point2 {
        Point2::new(self.x * s, self.y * s)
    }

    fn normalize(self) -> Point2 {
        self.scale(1.0 / self.norm())
    }
}

impl Add for Point2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2::new(
            self.x + other.x,
            self.y + other.y,
        )
    }
}

impl AddAssign for Point2 {
    fn add_assign(&mut self, other: Point2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point2 {
    type Output = Point2;

    fn sub(self, other: Point2) -> Point2 {
        Point2::new(
            self.x - other.x,
            self.y - other.y,
        )
    }
}

impl SubAssign for Point2 {
    fn sub_assign(&mut self, other: Point2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

fn to_point(step: &str) -> Point2 {
    // corners
    let a = [
        Point2::new( 1.0,  0.0),
        Point2::new( 1.0/2.0.sqrt(),  1.0/2.0.sqrt()),
        Point2::new(-1.0/2.0.sqrt(),  1.0/2.0.sqrt()),
        Point2::new(-1.0,  0.0),
        Point2::new(-1.0/2.0.sqrt(), -1.0/2.0.sqrt()),
        Point2::new( 1.0/2.0.sqrt(), -1.0/2.0.sqrt()),
    ];
    // scaled adjacent centers
    let direction = match step {
        "ne" => a[0] + a[1],
        "n"  => a[1] + a[2],
        "nw" => a[2] + a[3],
        "sw" => a[3] + a[4],
        "s"  => a[4] + a[5],
        "se" => a[5] + a[0],
        &_ => panic!("invalid step"),
    };
    direction
}

fn part_1(steps: &Vec<&str>) -> i64 {
    let target: Point2 = steps.iter()
        .map(|s| to_point(s))
        .fold(Point2::new(0.0, 0.0), |acc, x| acc + x);

    min_steps(target)
}

fn min_steps(target_in: Point2) -> i64 {
    let mut target = target_in.clone();

    let directions: Vec<Point2> = ["ne", "n", "nw", "sw", "s", "se"]
        .iter()
        .map(|s| to_point(s))
        .collect();
    for d in directions.iter() {
        println!("{:?}, {}", d, d.norm());
    }

    let mut count = 0;

    while target.norm() > 0.1 {
        let next_dir = directions.iter().max_by(|&&a, &&b| {
           let d_a = a.normalize().dot(target);
           let d_b = b.normalize().dot(target);
           d_a.partial_cmp(&d_b).unwrap()
        });
        match next_dir {
            Some(&d) => {
                target -= d;
            },
            None => return count,
        }
        count += 1;
    }
    count
}

fn part_2(steps: &Vec<&str>) -> i64 {
    let mut max_steps = 0;
    let mut current = Point2::new(0.0, 0.0);
    for s in steps.iter() {
        current += to_point(s);
        let current_steps = min_steps(current);
        if current_steps > max_steps {
            max_steps = current_steps;
        }
    }
    max_steps
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!(part_1(&vec!["ne","ne","ne"]), 3);
        assert_eq!(part_1(&vec!["ne","ne","sw", "sw"]), 0);
        assert_eq!(part_1(&vec!["ne","ne","s", "s"]), 2);
        assert_eq!(part_1(&vec!["se","sw","se", "sw", "sw"]), 3);
    }
}
