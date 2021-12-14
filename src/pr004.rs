use std::fmt::Display;

pub fn to_str<T>(n: T) -> String where
    T: ToString {
    n.to_string()
}

pub fn reverse_str(s: &String) -> String {
    s.chars().rev().collect()
}

pub fn is_pallindrome(s: &String) -> bool {
    *s == reverse_str(&s)
}

pub fn is_pallindrome_number<T>(n: T) -> bool where
    T:ToString {
    is_pallindrome(&to_str(n))
}

pub fn cproduct<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> where
    T: Copy, U: Copy {
    let mut ret = Vec::new();
    for a_ in a.iter() {
        for b_ in b.iter() {
            ret.push((*a_, *b_));
        }
    }

    ret
}

pub fn prod_vector<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> where
    T: Display + Copy + std::ops::Mul<Output = T> {
    cproduct(a, b).into_iter().map(|(a, b)| a * b).filter(|n| is_pallindrome_number(*n)).collect()
}


pub fn answer<T>(a: Vec<T>, b: Vec<T>) -> T where
    T: Ord + Display + Copy + std::ops::Mul<Output = T> {
    prod_vector(a, b).into_iter().max().unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_answer() {
        use super::answer;

        assert_eq!(answer((10..100).collect(), (10..100).collect()), 9009);
    }


    // #[test]
    // fn test_prod_vector() {
    //     use super::prod_vector;

    //     assert_eq!(prod_vector((100..1000).collect(), (100..1000).collect()), vec![]);
    // }


    // #[test]
    // fn test_cproduct() {
    //     use super::cproduct;

    //     assert_eq!(cproduct(vec![1,2,3,4], vec![1,2,3]), vec![]);
    // }


    #[test]
    fn test_str_int() {
        use super::to_str;

        assert_eq!(to_str(10), "10");
        assert_eq!(to_str(999), "999");
    }

    #[test]
    fn test_reverse_str() {
        use super::reverse_str;

        assert_eq!(reverse_str(&"a".to_string()), "a");
        assert_eq!(reverse_str(&"abc".to_string()), "cba");
    }

    #[test]
    fn test_is_pallindrome() {
        use super::is_pallindrome;

        assert_eq!(is_pallindrome(&"a".to_string()), true);
        assert_eq!(is_pallindrome(&"abc".to_string()), false);
        assert_eq!(is_pallindrome(&"abcba".to_string()), true);
    }

    #[test]
    fn test_is_pallindrome_number() {
        use super::is_pallindrome_number;

        assert_eq!(is_pallindrome_number(1), true);
        assert_eq!(is_pallindrome_number(11), true);
        assert_eq!(is_pallindrome_number(101), true);
        assert_eq!(is_pallindrome_number(909), true);
        assert_eq!(is_pallindrome_number(1123), false);
    }
}
