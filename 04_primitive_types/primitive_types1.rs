fn main() {
    let is_morning = true;
    
    if is_morning {
        println!("Good morning!");
    }

    // Defining `is_evening` as the negation of `is_morning`
    let is_evening = !is_morning;

    if is_evening {
        println!("Good evening!");
    }
}
