# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## My notes

- fn keyword, allows you to declare new functions
- snake case convention
- Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller (could be before or after the main if same file)
- We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. **Technically, the concrete values are called arguments**
- in function signatures, you must declare the type of each parameter (requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean)

## STATEMENTS AND EXPRESSIONS

- Statements are instructions that perform some action and do not return a value, therefore, you can’t assign a let statement to another variable. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6
  - Creating a variable and assigning a value to it with the let keyword is a statement.
  - Function definitions are also statements

```
let y = 6;
```

- Expressions evaluate to a resulting value.
  - Calling a function is an expression.
  - Calling a macro is an expression.
  - A new scope block created with curly brackets is an expression

##  Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust.

Adding a semicolon after 5 would produce an "mismatched types" error because:
The definition of the function five says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type. Therefore, nothing is returned, which contradicts the function definition and results in an error.
