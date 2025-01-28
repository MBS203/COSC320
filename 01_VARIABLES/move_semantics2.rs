fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

fn main() {
    // Creating a vector to test the function
    let vec0 = vec![22, 44, 66];
    
    // Calling the function
    let vec1 = fill_vec(vec0.clone());

    // Printing the original and modified vectors
    println!("Original vec0: {:?}", vec0);
    println!("Modified vec1: {:?}", vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
