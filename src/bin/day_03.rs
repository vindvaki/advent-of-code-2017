const INPUT: i32 = 368078;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    println!("{}", distance_from_center(INPUT));
}

fn part_2() {
    let mut spiral: Vec<i32> = vec![0, 1];
    for si in 2.. {
        let (i, j) = cartesian_index(si);
        let val = cartesian_neighbors(i, j).iter()
            .map(|&(ni, nj)| spiral_index(ni, nj))
            .filter(|&nsi| nsi < si)
            .fold(0, |acc, nsi| acc + spiral[nsi as usize]);
        spiral.push(val);
        if val > INPUT {
            break;
        }
    }
    if let Some(answer) = spiral.last() {
        println!("{}", answer)
    }
}

fn cartesian_neighbors(i: i32, j: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                result.push((i + x, j + y));
            }
        }
    }
    result
}

fn spiral_index(i: i32, j: i32) -> i32 {
    let r = i.abs().max(j.abs());
    let a = (2 * r + 1).pow(2);
    // [-r,   0,     r] == range of i
    // [ 0,   r,   2*r] == i + r

    // bottom
    if j == -r { return a + i - r; }

    // left
    if i == -r { return (a - 2*r) - j - r; }

    // top
    if j == r { return (a - 4*r) - i - r; }

    // right
    if i == r { return (a - 6*r) + j - r; }
    0
}

fn cartesian_index(n: i32) -> (i32, i32) {
    let r = ring_index(n);
    let (a, b, c, d) = ring_corners(r);
    if d - n >= 0 {
        // right
        return (r, n - d + r);
    }
    if c - n >= 0 {
        // top
        return (c - n - r, r);
    }
    if b - n >= 0 {
        // left
        return (-r, b - n - r);
    }
    if a - n >= 0 {
        // bottom
        return (n - a + r, -r);
    }
    (0, 0)
}

fn distance_from_center(n: i32) -> i32 {
    let i = ring_index(n);
    let c = nearest_corner(n);
    2 * i - (c - n).abs()
}

// ring sizes are squares with odd side length
//
// the lower right corner is the area of the spiral
// counting up to the corner
//
// it has the form (2*n + 1)**2 where the side length
// is 2*n + 1, and n is the index
fn ring_index(num: i32) -> i32 {
    ((num as f64).sqrt().ceil() as i32) / 2
}

// the corners of a ring, starting with the lower right corner
// in descending (clockwise) order
fn ring_corners(index: i32) -> (i32, i32, i32, i32) {
    let len = 2 * index + 1;
    let max = len * len;
    let step = len - 1;
    let a = max;
    let b = a - step;
    let c = b - step;
    let d = c - step;
    (a, b, c, d)
}

fn nearest_corner(n: i32) -> i32 {
    if n == 1 {
        return 1
    }
    let (a, b, c, d) = ring_corners(ring_index(n));
    let da = (n - a).abs();
    let db = (n - b).abs();
    let dc = (n - c).abs();
    let dd = (n - d).abs();
    let (_, corner) = (da,a).min((db,b)).min((dc,c)).min((dd,d));
    corner
}

#[cfg(test)]
mod tests {
    use cartesian_index;

    #[test]
    fn test_cartesian_index_1() {
        // center
        assert_eq!(cartesian_index(1), (0,0));
    }

     #[test]
    fn test_cartesian_index_12() {
        // right
        assert_eq!(cartesian_index(12), (2,1));
    }

     #[test]
    fn test_cartesian_index_14() {
        // top
        assert_eq!(cartesian_index(14), (1,2));
    }

    #[test]
    fn test_cartesian_index_20() {
        // left
        assert_eq!(cartesian_index(20), (-2,-1));
    }

     #[test]
    fn test_cartesian_index_22() {
        // bottom
        assert_eq!(cartesian_index(22), (-1,-2));
    }

    use spiral_index;

    #[test]
    fn test_spiral_index_1() {
        // center
        assert_eq!(spiral_index(0, 0), 1);
    }

    #[test]
    fn test_spiral_index_2() {
        // center
        assert_eq!(spiral_index(1, 0), 2);
    }

    #[test]
    fn test_spiral_index_7() {
        // corner
        assert_eq!(spiral_index(-1, -1), 7);
    }

    #[test]
    fn test_spiral_index_10() {
        // right
        assert_eq!(spiral_index(2, -1), 10);
    }

    #[test]
    fn test_spiral_index_14() {
        // top
        assert_eq!(spiral_index(1, 2), 14);
    }

    #[test]
    fn test_spiral_index_18() {
        // left
        assert_eq!(spiral_index(-2, 1), 18);
    }

    #[test]
    fn test_spiral_index_22() {
        // bottom
        assert_eq!(spiral_index(-1, -2), 22);
    }

    use distance_from_center;

    #[test]
    fn test_distance_from_center_1() {
        assert_eq!(distance_from_center(1), 0);
    }

    #[test]
    fn test_distance_from_center_2() {
        assert_eq!(distance_from_center(2), 1);
    }

    #[test]
    fn test_distance_from_center_9() {
        assert_eq!(distance_from_center(9), 2);
    }

    #[test]
    fn test_distance_from_center_7() {
        assert_eq!(distance_from_center(7), 2);
    }

    #[test]
    fn test_distance_from_center_19() {
        assert_eq!(distance_from_center(19), 2);
    }

    #[test]
    fn test_distance_from_center_20() {
        assert_eq!(distance_from_center(20), 3);
    }

    #[test]
    fn test_distance_from_center_21() {
        assert_eq!(distance_from_center(21), 4);
    }

    use nearest_corner;

    #[test]
    fn test_nearest_corner_2() {
        assert_eq!(nearest_corner(2), 3);
    }

    #[test]
    fn test_nearest_corner_7() {
        assert_eq!(nearest_corner(7), 7);
    }

    #[test]
    fn test_nearest_corner_20() {
        assert_eq!(nearest_corner(20), 21);
    }

    use ring_corners;

    #[test]
    fn test_ring_corners_1() {
        assert_eq!(ring_corners(1), (9, 7, 5, 3))
    }

    #[test]
    fn test_ring_corners_2() {
        assert_eq!(ring_corners(2), (25, 21, 17, 13))
    }

    use ring_index;

    #[test]
    fn test_ring_index_1() {
        assert_eq!(ring_index(1), 0);
    }

    #[test]
    fn test_ring_index_3() {
        assert_eq!(ring_index(3), 1);
    }

    #[test]
    fn test_ring_index_5() {
        assert_eq!(ring_index(5), 1);
    }

    #[test]
    fn test_ring_index_9() {
        assert_eq!(ring_index(9), 1);
    }

    #[test]
    fn test_ring_index_10() {
        assert_eq!(ring_index(10), 2);
    }

    #[test]
    fn test_ring_index_25() {
        assert_eq!(ring_index(25), 2);
    }

    #[test]
    fn test_ring_index_26() {
        assert_eq!(ring_index(26), 3);
    }
}
