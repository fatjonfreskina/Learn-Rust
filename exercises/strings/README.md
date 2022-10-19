# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

### Notes 

Many of the same operations available with Vec<T> are available with String as well, because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.

```rust
let mut s = String::new();      // heap allocated String
let data = "initial contents";  // string literal
let s1 = data.to_string();       // conversion from lit to String
let s2 = String::from("initial contents"); // heap allocated string from literal

// Update of string
s.push_str("bar");              
s1.push_str(s2);                
let s3 = s1 + &s2; // s1 ownership is passed to s3

// String formatting
let s = format!("{}-{}-{}", s1, s2, s3); // s contains a formatted string
```

### Rust strings don’t support indexing

A String is a wrapper over a Vec\<u8>.

```rust
let hello = String::from("Hola");           // len = 4
let hello = String::from("Здравствуйте");   // len = 24 .. wtf?
```

Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, s will be a &str that contains the first 4 bytes of the string.

### Methods for Iterating Over Strings

The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.

For individual Unicode scalar values, use the chars method.

```rust
for c in "Зд".chars() {
    println!("{}", c);
}
```

Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:

```rust
for b in "Зд".bytes() {
    println!("{}", b);
}

// 208
// 151
// 208
// 180
```

