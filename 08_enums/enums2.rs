#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Resize { width, height } => {
                println!("Resize: width={}, height={}", width, height);
            }
            Message::Move(Point { x, y }) => {
                println!("Move: x={}, y={}", x, y);
            }
            Message::Echo(message) => {
                println!("Echo: {}", message);
            }
            Message::ChangeColor(r, g, b) => {
                println!("ChangeColor: r={}, g={}, b={}", r, g, b);
            }
            Message::Quit => {
                println!("Quit");
            }
        }
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
