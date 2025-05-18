enum Color {
    Red,
    Green,
    Blue
}

struct RGB(u8, u8, u8);

fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255)
    }
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String)
}

fn handle_message(message: Message) {
    match message {
        Message::StartGame => println!("The game is starting!"),
        Message::WinPoint { who } => println!("The point will look out: {}", who),
        Message::ChangePlayerName(name) => println!("Changed player name: {}", name)
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x { 
        Some(i) => Some(i + 1),
        None => None
    }
}
fn main() {
    let x = Some(2);
    println!("{:?}", increment(x));
    println!("{:?}", increment(None));
}
