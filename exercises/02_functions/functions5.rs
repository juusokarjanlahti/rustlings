// TODO: Fix the function body without changing the signature.


/*
We want to return a value with the type `i32` from the `square` function, but
it is returning the type `()`.

There are two solutions:
1. Add the `return` keyword before `num * num;`
2. Remove the semicolon `;` after `num * num`
*/
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}