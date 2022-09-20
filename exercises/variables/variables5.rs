// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// Shadowing: the first variable is shadowed by the second, 
// which means that the second variable is what the compiler 
// will see when you use the name of the variable.  
// See: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
