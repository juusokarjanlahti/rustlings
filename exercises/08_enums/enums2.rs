#[derive(Debug)]

// define a struct named Point with two u64 fields: x and y
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
// define enum Message with variants
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u64, height: u64 }, 
    // we use {} to define a struct with fields width and height inside the Resize enum variant
    Move(Point),
    // then we have Move, echo, ChangeColor and Quit which take different types of data
    Echo(String),
    ChangeColor(u64, u64, u64),
    Quit,
}

// implement Message with a method named call that prints the message from the variant
// by calling println!("{self:?}"); where self is the enum variant
impl Message {
    fn call(&self) {
        println!("{self:?}");
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
