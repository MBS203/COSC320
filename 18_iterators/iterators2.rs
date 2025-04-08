// Capitalize the first letter of a string
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Apply capitalize_first to a slice of &str and return a vector of capitalized strings
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Apply capitalize_first to a slice of &str and return a single joined string
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect::<String>()
}

fn main() {
    // Example usage to avoid "function not used" warnings
    println!("{}", capitalize_first("example")); // Output: Example

    let vec = capitalize_words_vector(&["foo", "bar"]);
    println!("{:?}", vec); // Output: ["Foo", "Bar"]

    let joined = capitalize_words_string(&["hello", " ", "there"]);
    println!("{}", joined); // Output: Hello There
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
