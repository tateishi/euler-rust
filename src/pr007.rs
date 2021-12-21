use crate::prime::maybe_primenumber;


pub fn is_prime(n: u64) -> bool {
    if n == 1 { return false; }
    for p in maybe_primenumber().take_while(|x| *x * *x <= n) {
        if n % p == 0 {
            return false;
        }
    }
    true
}

pub struct Primes {
    last: u64,
}


impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.last) {
            self.last += 1;
        }
        let p = self.last;
        self.last += 1;
        Some(p)
    }
}


pub fn primes() -> Primes {
    Primes { last: 1 }
}


pub fn answer(n: u64) -> u64 {
    let mut ps = primes().skip((n - 1).try_into().unwrap());

    ps.next().unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_prime() {
        use super::primes;

        let  ps = primes();

        assert_eq!(ps.skip(10000).take(1).collect::<Vec<u64>>(), [104743]);
    }

    #[test]
    fn test_answer() {
        use super::answer;

        assert_eq!(answer(6), 13);
        assert_eq!(answer(7), 17);
        assert_eq!(answer(10), 29);
    }
}
