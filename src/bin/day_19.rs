extern crate aoc2017;
use aoc2017::read_fixture;

type Board = Vec<Vec<char>>;

fn main() {
    let input = read_fixture();
    println!("part_1: {}", part_1(input.as_str()));
    println!("part_2: {}", part_2(input.as_str()));
}

fn new_board(input: &str) -> Board {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> String {
    let (s, _) = solve(input);
    s
}

fn part_2(input: &str) -> usize {
    let (_, s) = solve(input);
    s
}

fn solve(input: &str) -> (String, usize) {
    let board = new_board(input);
    let mut path = String::new();
    let mut count = 0;
    let rows = board.len();
    let cols = board[0].len();

    let mut i = 0;
    let mut j = match (0..cols).find(|&j| board[0][j] == '|') {
        Some(v) => v,
        None => panic!("Invalid board!"),
    };
    let mut direction = 'v';
    loop {
        match board[i][j] {
            // terminating condition
            ' ' => break,
            // seek next direction
            '+' => {
                match direction {
                    'v' | '^' => {
                        if j > 0 && (board[i][j - 1] == '-' || board[i][j - 1].is_alphabetic()) {
                            direction = '<';
                        }
                        if j < cols - 1 && (board[i][j + 1] == '-' || board[i][j + 1].is_alphabetic()) {
                            direction = '>';
                        }
                    },
                    '>' | '<' => {
                        if i > 0 && (board[i - 1][j] == '|' || board[i - 1][j].is_alphabetic()) {
                            direction = '^'
                        }
                        if i < rows - 1 && (board[i + 1][j] == '|' || board[i + 1][j].is_alphabetic()) {
                            direction = 'v'
                        }
                    },
                    _ => panic!("invalid direction"),
                };
            },
            // same direction
            '|' | '-' => {},
            // alphanumeric
            _ => path.push(board[i][j]),
        };
        match direction {
            'v' => {
                if i == rows - 1{
                    break;
                } else {
                    i += 1;
                }
            },
            '^' => {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            },
            '>' => {
                if j == cols - 1 {
                    break;
                } else {
                    j += 1;
                }
            },
            '<' => {
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            },
            _ => panic!("invalid direction"),
        };
        count += 1;
    }
    (path, count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        let input =[
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
        ].join("\n");
        assert_eq!(part_1(input.as_str()), "ABCDEF".to_owned());
    }

    #[test]
    fn test_part_2() {
        use part_2;
        let input =[
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
        ].join("\n");
        assert_eq!(part_2(input.as_str()), 38);
    }
}
