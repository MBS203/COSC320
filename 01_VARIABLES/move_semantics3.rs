fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    // Create a vector
    let vec0 = vec![22, 44, 66];

    // Pass the cloned vector to the function and get the modified vector
    let vec1 = fill_vec(vec0.clone());

    // Print the original and modified vectors
    println!("Original vec0: {:?}", vec0); // vec0 is still available here
    println!("Modified vec1: {:?}", vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

