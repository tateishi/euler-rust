pub mod pr001 {
    pub fn mod_3_or_5(n: i32) -> bool {
        match (n % 3, n % 5) {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }

    pub fn answer(n: i32) -> i32 {
        (1..n).filter(|&x| mod_3_or_5(x)).sum()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_mod_3_or_5() {
            use super::mod_3_or_5;

            assert_eq!(mod_3_or_5(1), false);
            assert_eq!(mod_3_or_5(2), false);
            assert_eq!(mod_3_or_5(3), true);
            assert_eq!(mod_3_or_5(4), false);
            assert_eq!(mod_3_or_5(5), true);
            assert_eq!(mod_3_or_5(6), true);
            assert_eq!(mod_3_or_5(7), false);
            assert_eq!(mod_3_or_5(8), false);
            assert_eq!(mod_3_or_5(9), true);
            assert_eq!(mod_3_or_5(10), true);
        }

        #[test]
        fn test() {
            use super::answer;

            assert_eq!(answer(10), 23);
        }
    }
}


pub mod pr002 {
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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}
