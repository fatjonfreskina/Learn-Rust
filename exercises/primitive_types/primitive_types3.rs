// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = [0;101]; // semicolon for quick initialization of big arrays

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("{}", a.len());
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        println!("{}", a.len());

    }
}
