pub struct Fib {
    curr: i32,
    next: i32,
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn fib() -> Fib {
    Fib { curr: 0, next: 1 }
}

pub fn answer(n: i32) -> i32 {
    fib().take_while(|x| *x < n).filter(|x| *x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibbonacci() {
        use super::fib;

        let mut seq0 = fib().take_while(|x| *x < 10);

        assert_eq!(seq0.next(), Some(1));
        assert_eq!(seq0.next(), Some(1));
        assert_eq!(seq0.next(), Some(2));
        assert_eq!(seq0.next(), Some(3));
        assert_eq!(seq0.next(), Some(5));
        assert_eq!(seq0.next(), Some(8));
        assert_eq!(seq0.next(), None);


        let mut seq1 = fib().take_while(|x| *x < 10).filter(|x| *x % 2 == 0);

        //            assert_eq!(seq1.next(), Some(1));
        //            assert_eq!(seq1.next(), Some(1));
        assert_eq!(seq1.next(), Some(2));
        //            assert_eq!(seq1.next(), Some(3));
        //            assert_eq!(seq1.next(), Some(5));
        assert_eq!(seq1.next(), Some(8));
        assert_eq!(seq1.next(), None);
    }

    #[test]
    fn test_answer() {
        use super::answer;

        assert_eq!(answer(10), 10);
        assert_eq!(answer(20), 10);
        assert_eq!(answer(50), 44);
    }
}
