// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");
    // string_slice can take a string slice, so we can use string_slice

    string("red".to_string());
    // to_string() returns a new String, so we can use string

    string(String::from("hi"));
    // to_string() returns a new String, so we can use string

    string("rust is fun!".to_owned());
    // to_owned() returns a new String, so we can use string

    string_slice("nice weather".into());
    // into() converts a string literal into a String, so we can use string_slice

    string(format!("Interpolation {}", "Station"));
    // format! returns a new String, so we can use string

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);
    // we can use string_slice because we are taking a slice of a string

    string_slice("  hello there ".trim());
    // trim() returns a new string slice, so we can use string_slice

    string("Happy Monday!".replace("Mon", "Tues"));
    // replace() returns a new String, so we can use string

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    // to_lowercase() returns a new String, so we can use string
}
