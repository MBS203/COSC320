fn main() {
    // Note the _single_ quotes for characters
    let my_first_initial = 'C';

    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // Declaring `your_character` with different possibilities
    let your_character = '😊'; // Try changing this to a letter, number, or special character

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
