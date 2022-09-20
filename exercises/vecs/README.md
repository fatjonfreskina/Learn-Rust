# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)

## Notes 

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.

Creating a vector
```rust
fn main() {
    let v: Vec<i32> = Vec::new();
}
```
Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store

More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation. Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it

```rust
fn main() {
    let v = vec![1, 2, 3];
}

```
Updating a vector

```rust
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

Reading elements: two ways!
```rust
let v = vec![1, 2, 3, 4, 5];

// First way, using the reference of v[index]
let third: &i32 = &v[2]; // Reference to the third element
println!("The third element is {}", third);

// Second way, using the get method
let third: Option<&i32> = v.get(2); 
//  When we use the get method with the index passed as an argument, 
//  we get an Option<&T> that we can use with match.
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];       // if you want the program to panic (crash)
    let does_not_exist = v.get(100);    // returns none without panicking
}
```

## Iterating on a vector

```rust
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
```

## Iterate and change values

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // deference operator
    }
}
```