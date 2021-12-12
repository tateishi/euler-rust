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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pr001_mod_3_or_5() {
        use super::pr001::mod_3_or_5;

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
    fn test_pr001() {
        use super::pr001::answer;

        assert_eq!(answer(10), 23);
    }
}
