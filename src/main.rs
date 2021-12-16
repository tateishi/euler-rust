fn main() {
    use euler::pr001;
    use euler::pr002;
    use euler::pr003;
    use euler::pr004;
    use euler::pr005;

    println!("Problem 001, the answer is {}", pr001::answer(1000));
    println!("Problem 002, the answer is {}", pr002::answer(40000000));
    println!("Problem 003, the answer is {}", pr003::answer(600851475143));
    println!("Problem 004, the answer is {}", pr004::answer((100..1000).collect(), (100..1000).collect()));
    println!("{:?}", pr005::factors(999));
}
