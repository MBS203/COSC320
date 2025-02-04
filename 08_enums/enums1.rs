#[derive(Debug)]
enum Message {
    Resize(u32, u32),
    Move(u32, u32),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    // Creating instances of different Message variants.
    let resize_message = Message::Resize(800, 600);
    let move_message = Message::Move(10, 20);
    let echo_message = Message::Echo(String::from("Hello"));
    let color_message = Message::ChangeColor(255, 0, 0);
    let quit_message = Message::Quit;

    // Matching on the Message enum variants and using the fields.
    match resize_message {
        Message::Resize(width, height) => println!("Resizing to {}x{}", width, height),
        _ => (),
    }

    match move_message {
        Message::Move(x, y) => println!("Moving to ({}, {})", x, y),
        _ => (),
    }

    match echo_message {
        Message::Echo(ref text) => println!("Echoing: {}", text),  // Borrow the String
        _ => (),
    }

    match color_message {
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        _ => (),
    }

    match quit_message {
        Message::Quit => println!("Quit command"),
        _ => (),
    }

    // Printing the different variants of the `Message` enum using the `Debug` trait.
    println!("{:?}", resize_message);
    println!("{:?}", move_message);
    println!("{:?}", echo_message);  // No longer a move, since `text` was borrowed
    println!("{:?}", color_message);
    println!("{:?}", quit_message);
}
