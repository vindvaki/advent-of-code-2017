extern crate aoc2017;

use aoc2017::read_fixture;

fn main() {
    let data = read_fixture();
    part_1(&data.as_str());
    part_2(&data.as_str());
}

fn part_1(data: &str) {
    let solution = data.lines().filter(|&line| !contains_duplicate_words(line)).count();
    println!("part_1: {}", solution);
}

fn part_2(data: &str) {
    let solution = data.lines().filter(|&line| !contains_anagrams(line)).count();
    println!("part_2: {}", solution);
}

fn contains_duplicate_words(line: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for word in line.split_whitespace() {
        if seen.contains(word) {
            return true;
        }
        seen.insert(word);
    }
    false
}

fn contains_anagrams(line: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for word in line.split_whitespace() {
        let sorted = {
            let mut res: Vec<char> = word.chars().collect();
            res.sort();
            res
        };
        if seen.contains(&sorted) {
            return true;
        }
        seen.insert(sorted);
    }
    false
}

#[cfg(test)]
mod tests {
    use contains_anagrams;

    #[test]
    fn test_contains_anagrams_with_anagrams() {
        assert_eq!(contains_anagrams("abc r cba b d"), true);
    }

    #[test]
    fn test_contains_anagrams_without_anagrams() {
        assert_eq!(contains_anagrams("aabc r cba b d"), false);
    }

    use contains_duplicate_words;

    #[test]
    fn test_contains_duplicate_words_with_duplicates() {
        assert_eq!(contains_duplicate_words("a abc b abc"), true);
    }

    #[test]
    fn test_contains_duplicate_words_without_duplicates() {
        assert_eq!(contains_duplicate_words("a b c"), false);
    }
}
