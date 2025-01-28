fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88); // Mutate the vector
    vec
}

fn main() {
    let vec0 = vec![10, 20, 30]; // Initial vector
    let vec1 = fill_vec(vec0); // Call the function to modify the vector
    println!("Modified vector: {:?}", vec1); // Print the modified vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
