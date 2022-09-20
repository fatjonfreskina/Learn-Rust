# Primitive Types

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)

## Notes

## Primitive scalar types

Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

Integers (i or u): i8,i16,i32, i64, i128, isize (isize is called "arch" type, depends on architecture)

floating-point: f32, f64

Booleans: bool

Characters: char

## Compound Types

Two compound primary types: tuples and arrays

### TUPLES

Tuples have a fixed length: once declared, they cannot grow or shrink in size. We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. 

```rust
fn main() {
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // Destructing the tuple

    println!("The value of y is: {y}");
}
```
Accessing with `.`:
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
### ARRAYS

Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
Or:
```rust
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```
Or:
```rust
fn main() {
let a = [3; 5]; // [3,3,3,3,3,]

let first = a[0]; // access
let second = a[1];
}
```

## Slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.