#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.

    // When we use the `match` statement, we can destructure the `Point` struct
    // and access its fields directly. However, since `Point` does not implement the `Copy` trait,
    // we need to use a reference to avoid moving the value out of the `Option`.
    // Ref binds the value to a variable, and allows us to use it without moving it.

    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
