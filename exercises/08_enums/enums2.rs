// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

///

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Echo(msg) => println!("Echo: {}", msg),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
            Message::Quit => println!("Quit"),
        }
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
