
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


#[derive(Debug)]
pub struct PrimeCount {
    prime: u64,
    count: usize,
}


impl PrimeCount {
    pub fn new(p: u64) -> PrimeCount {
        PrimeCount {
            prime: p,
            count: 1,
        }
    }

    pub fn add(&self, n: u64) -> PrimeCount {
        if self.prime == n {
            PrimeCount {
                prime: self.prime,
                count: self.count + 1,
            }
        } else {
            PrimeCount {
                prime: self.prime,
                count: self.count,
            }
        }
    }

    pub fn to_number(&self) -> u64 {
        use std::iter;

        iter::repeat(self.prime).take(self.count).product()
    }
}


pub fn factors(n: u64) -> Vec<PrimeCount> {
    let maybe_prime = maybe_primenumber();
    let mut ret = Vec::new();
    let mut value = n;

    for p in maybe_prime.take_while(|x| *x <= n) {
        if value == 1 {
            break;
        }
        if value % p == 0 {
            ret.push(PrimeCount::new(p));
            value /= p;
        }
        while value % p == 0 {
            ret = ret.into_iter().map(|pc| pc.add(p)).collect();
            value /= p;
        }
    }

    ret
}


pub fn to_number(v: Vec<PrimeCount>) -> u64 {
    let mut ret: u64 = 1;

    for pc in v.into_iter() {
        ret *= pc.to_number();
    }

    ret
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        use super::PrimeCount;

        let pc = PrimeCount::new(2);

        assert_eq!(pc.prime, 2);
        assert_eq!(pc.count, 1);
    }

    #[test]
    fn test_add() {
        use super::PrimeCount;

        let pc = PrimeCount::new(2);
        let pc2 = pc.add(2);
        assert_eq!(pc2.prime, 2);
        assert_eq!(pc2.count, 2);

        let pc3 = pc.add(2).add(3);
        assert_eq!(pc3.prime, 2);
        assert_eq!(pc3.count, 2);
    }

    #[test]
    fn test_to_number() {
        use super::PrimeCount;

        let pc = PrimeCount::new(2);
        let pc2 = pc.add(2);
        assert_eq!(pc2.to_number(), 4);

        let pc = PrimeCount::new(3);
        let pc2 = pc.add(3);
        assert_eq!(pc2.to_number(), 9);
    }

    #[test]
    fn test_factors() {
        use super::factors;
        use super::to_number;

        let facts = factors(12);
        assert_eq!(facts[0].to_number(), 4);
        assert_eq!(facts[1].to_number(), 3);
        assert_eq!(to_number(facts), 12);
    }

}
