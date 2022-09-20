// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.

/*
 - Rust’s naming convention for constants is to use all uppercase with underscores between words
 - Constants are valid for the entire time a program runs, within the scope they were declared in
 -  the type of the value must be annotated
 */


 const NUMBER : i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
