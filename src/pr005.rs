use std::collections::HashMap;
use std::cmp::max;


const WHEEL30: [u64; 8] = [7, 11, 13, 17, 19, 23, 29, 31];

pub struct Wheel30 {
    index: usize,
    base: u64,
}

impl Wheel30 {
    pub fn new() -> Wheel30 {
        Wheel30 {
            index: 0, base: 0,
        }
    }
}

impl Iterator for Wheel30 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.base + WHEEL30[self.index];
        self.index += 1;
        if self.index > WHEEL30.len() - 1 {
            self.index = 0;
            self.base += 30;
        }
        Some(value)
    }
}


pub fn maybe_primenumber() -> impl Iterator<Item=u64> {
    [2, 3, 5].into_iter().chain(Wheel30::new())
}


pub fn factors(n: u64) -> HashMap<u64, u64> {
    let maybe_prime = maybe_primenumber();
    let mut fact = HashMap::new();
    let mut value = n;

    for p in maybe_prime.take_while(|x| *x <= n) {
        if value == 1 {
            break;
        }

        while value % p == 0 {
            let e = fact.entry(p).or_insert(0_u64);
            *e += 1;
            value /= p;
        }
    }
    fact
}


pub fn max_factors(a: &mut HashMap<u64, u64>, b: &HashMap<u64, u64>) -> () {
    for (k, v) in b {
        let e = a.entry(*k).or_insert(*v);
        *e = max(*e, *v);
    }
}


pub fn to_number(h: &HashMap<u64, u64>) -> u64 {
    h.iter().map(|(k, v)| k.pow(*v as u32)).product()
}


pub fn answer(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut fact = factors(2);
    for m in 3..=n {
        max_factors(&mut fact, &factors(m));
    }
    to_number(&fact)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_factors() {
        use super::factors;
        use super::max_factors;
        use super::to_number;


        let mut a = factors(2);
        println!("{:?}", a);
        for n in 3..=10 {
            max_factors(&mut a, &factors(n));
        }

        assert_eq!(to_number(&a), 2520);
    }

    #[test]
    fn test_lcm() {
        use super::answer;

        assert_eq!(answer(1), 1);
        assert_eq!(answer(2), 2);
        assert_eq!(answer(3), 6);
        assert_eq!(answer(10), 2520);
    }
}
