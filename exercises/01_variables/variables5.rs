fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // Variables can be overwritten with shadowing.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

// SHADOWING, creating a variable with the same name
// but a new value and type 
// OVERWRITE/UPDATE/REASSIGN, updates only the value (needs mutability)

/*  
THESE DON'T WORK:
let mut number = "T-H-R-E-E";
number = 3;
or
let number = "T-H-R-E-E";
number = "3";

DOES WORK:
let number = "T-H-R-E-E";
let number = 3;
*/