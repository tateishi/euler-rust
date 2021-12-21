use crate::prime::maybe_primenumber;


pub fn factors(n: u64) -> Vec<u64> {
    let maybe_prime = maybe_primenumber();
    let mut ret = Vec::new();
    let mut value = n;

    for p in maybe_prime.take_while(|x| *x <= n) {
        if value == 1 {
            break;
        }
        while value % p == 0 {
            ret.push(p);
            value /= p;
        }
    }

    ret
}


pub fn answer(n: u64) -> u64 {
    factors(n).into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_wheel30() {
        use super::maybe_primenumber;

        let mut iter = maybe_primenumber();

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(17));
        assert_eq!(iter.next(), Some(19));
        assert_eq!(iter.next(), Some(23));
        assert_eq!(iter.next(), Some(29));
        assert_eq!(iter.next(), Some(31));
        assert_eq!(iter.next(), Some(37));
        assert_eq!(iter.next(), Some(41));
    }


    #[test]
    fn test_factors() {
        use super::factors;

        assert_eq!(factors(1), []);
        assert_eq!(factors(2), [2]);
        assert_eq!(factors(3), [3]);
        assert_eq!(factors(4), [2, 2]);
        assert_eq!(factors(5), [5]);
        assert_eq!(factors(6), [2, 3]);
        assert_eq!(factors(7), [7]);
        assert_eq!(factors(8), [2, 2, 2]);
        assert_eq!(factors(9), [3, 3]);
        assert_eq!(factors(10), [2, 5]);

        assert_eq!(factors(13195), [5, 7, 13, 29]);

        assert_eq!(factors(600851475143), [71, 839, 1471, 6857]);
    }
}
