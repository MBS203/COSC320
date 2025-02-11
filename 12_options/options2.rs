fn main() {
    println!("Main function is running.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            println!(" Found word: {}", word); // Added print statement
            assert_eq!(word, target);
        } else {
            println!(" No word found!");
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        while let Some(Some(integer)) = optional_integers.pop() {
            println!(" Popped integer: {}", integer); // Added print statement
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        println!(" Final cursor value: {}", cursor); // Added print statement
        assert_eq!(cursor, 0);
    }
}
