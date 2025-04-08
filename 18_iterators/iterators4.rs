fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

fn main() {
    println!("Factorial of 0 is {}", factorial(0));
    println!("Factorial of 1 is {}", factorial(1));
    println!("Factorial of 2 is {}", factorial(2));
    println!("Factorial of 4 is {}", factorial(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
