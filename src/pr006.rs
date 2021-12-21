
pub fn sum_of_square(n: u64) -> u64 {
    (1..=n).map(|x| x.pow(2)).sum()
}


pub fn square_of_sum(n: u64) -> u64 {
    (1..=n).sum::<u64>().pow(2)
}


pub fn answer(n: u64) -> u64 {
    square_of_sum(n) - sum_of_square(n)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_of_square() {
        use super::sum_of_square;

        assert_eq!(sum_of_square(10), 385);
    }

    #[test]
    fn test_square_of_sum() {
        use super::square_of_sum;

        assert_eq!(square_of_sum(10), 3025);
    }

    #[test]
    fn test_answer() {
        use super::answer;

        assert_eq!(answer(10), 2640);
    }
}
