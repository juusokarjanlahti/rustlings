struct Point { // define a struct named Point with two u64 fields: x and y
    x: u64,
    y: u64,
}

enum Message { // define enum Message with variants with various data types
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State { // define a struct named State with fields width, height, position, message, color and quit
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State { // implement State struct with methods to resize, move_position, echo, change_color and quit
    // fn resize takes a mutable reference to self and the width and height as arguments
    fn resize(&mut self, width: u64, height: u64) {
        // then it sets the width and height fields of the struct to the width and height arguments
        self.width = width;
        self.height = height;
    }

    // fn move_position takes a mutable reference to self and a Point as argument
    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String){
        self.message = s;
    }

    // fn change_color takes a mutable reference to self and the red, green and blue values as arguments
    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    // fn quit takes a mutable reference to self and sets the quit field to true
    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        match message { // match expression called message
            Message::Resize { width, height } => self.resize(width, height), // if message is Resize, call resize method
            Message::Move(point) => self.move_position(point), // if message is Move, call move_position method
            Message::Echo(s) => self.echo(s), // if message is Echo, call echo method
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue), // if message is ChangeColor, call change_color method
            Message::Quit => self.quit(), // if message is Quit, call quit method
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
// mod tests are used to group related tests together
mod tests {
    // use super::*; is used to bring the functions and structs defined in the parent module into the current module
    use super::*;

    // #[test] is an attribute used to define a test function
    // this is our first one and it tests the process method of the State struct
    #[test]
    // the test function is named test_match_message_call
    fn test_match_message_call() {
        // creaate mutable state variable and initialize it with a State struct 
        let mut state = State {
            // set the width, height, position, message, color and quit fields of the struct
            // we can set them whatever we want since we are going to change them with the process method
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        // call the process method of the mutable state variable we just created with different Message variants
        // passing Message::Resize will call the resize method of the State struct
        // and let us set the width and height fields of the struct variable
        // we process the other Message variants in the same way while and passing correct values based on the variant
        // if we passed wrong values or the wrong variant, the test would fail and we would get an error
        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        // then we use assert_eq! macro that to check if the fields were set correctly for the different Message variants
        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
