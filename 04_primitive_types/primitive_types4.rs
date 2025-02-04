fn main() {
    let a = [1, 2, 3, 4, 5];

    // Extracting a slice from the array
    let nice_slice = &a[1..4];

    // Printing the extracted slice
    println!("Extracted slice: {:?}", nice_slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // Extracting the required slice
        let nice_slice = &a[1..4];

        assert_eq!(&[2, 3, 4], nice_slice);
    }
}
