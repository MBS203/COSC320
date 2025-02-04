fn main() {
    // You can optionally experiment here.
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Test function to check indexing into a tuple
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Accessing the second element of the tuple using index 1
        let second = numbers.1;

        // Assert the second element is 2
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }

    // Add another simple test function to make sure something else is also running
    #[test]
    fn another_test() {
        assert_eq!(2 + 2, 4, "Simple math test failed");
    }
}
