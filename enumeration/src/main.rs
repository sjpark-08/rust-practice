#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String)
}

// enum Option<T> {
//     None,
//     Some(T)
// }
fn main() {
    let red = Color::Red;
    let green = Color::Green;

    println!("red = {:?}", red);
    println!("red == green => {}", red == green);
    println!("red == red => {}", red == Color::Red);
    
    let m1 = Message::StartGame;
    let m2 = Message::WinPoint {
        who: String::from("hong")
    };
    let m3 = Message::ChangePlayerName(String::from("James"));
    
    let some_number = Some(2);
    let absent_number: Option<i32> = None;
    
}
