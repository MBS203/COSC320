fn main() {
    // Creating an array `a` with 100 elements, all initialized to 0
    let a = [0; 100]; // This creates an array with 100 elements, all set to 0.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
