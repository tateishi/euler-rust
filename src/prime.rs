
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


#[cfg(test)]
mod tests {
    #[test]
    fn test_maybe_primenumber() {
        use super::maybe_primenumber;


//        println!("{:?}", maybe_primenumber().take(5).collect::<Vec<u64>>());
//        println!("{:?}", maybe_primenumber().skip(15).take(5).collect::<Vec<u64>>());

        assert_eq!(maybe_primenumber().take(5).collect::<Vec<u64>>(), [2, 3, 5, 7, 11]);
        assert_eq!(maybe_primenumber().skip(15).take(5).collect::<Vec<u64>>(), [49, 53, 59, 61, 67]);
    }
}
