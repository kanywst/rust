struct Point { x: f64, y: f64 }

enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move {},{}", x, y),
        Message::Write(s) => println!("{}", s),
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("Point: ({}, {})", p.x, p.y);

    process(Message::Write(String::from("hello")));
}
