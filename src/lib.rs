pub struct Digits(u64, bool);

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 && self.1 {
            return None;
        }
        self.1 = true;
        let d = self.0 % 10;
        self.0 /= 10;
        Some(d)
    }
}

pub fn digits(n: u64) -> Digits {
    Digits(n, false)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_digits() {
        use digits;
        // 0 is 0
        let mut digits_0 = digits(0);
        assert_eq!(digits_0.next(), Some(0));

        let mut digits_1234 = digits(1234);
        assert_eq!(digits_1234.next(), Some(4));
        assert_eq!(digits_1234.next(), Some(3));
        assert_eq!(digits_1234.next(), Some(2));
        assert_eq!(digits_1234.next(), Some(1));
    }
}

