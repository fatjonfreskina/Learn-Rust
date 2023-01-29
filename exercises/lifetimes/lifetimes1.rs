// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a hint.

/*
IDEA:
When we’re defining this function, we don’t know the concrete values 
that will be passed into this function, so we don’t know whether the 
if case or the else case will execute. We also don’t know the concrete 
lifetimes of the references that will be passed in, so we can’t look at 
the scopes as we did in Listings 10-17 and 10-18 to determine whether 
the reference we return will always be valid. The borrow checker can’t 
determine this either, because it doesn’t know how the lifetimes of x 
and y relate to the lifetime of the return value. To fix this error, 
we’ll add generic lifetime parameters that define the relationship 
between the references so the borrow checker can perform its analysis.
*/


// constraint: the returned reference 
// will be valid as long as both the parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
