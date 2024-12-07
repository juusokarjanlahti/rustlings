fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // pattern to bind attributes to appropriate parts of the cat tuple:
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
