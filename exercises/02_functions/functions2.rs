// TODO: Add the missing type of the argument `num` after the colon `:`.

// Function's signature has to have type annotations for all arguments and return value.
fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
