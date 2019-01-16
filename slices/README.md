# The Slice Type

A _string slice_ is a reference to a part of a `String`. it is usually created with the `&[start..end]` syntax. The resulting string ia a reference to a part of the original string.

For example:

```rust
    let name = String::from("Timothy"); // Timothy
    let nameSlice = &[0..3]; // Tim
    let nameSlice2 = &[0..4]; // Timo
```

The `start` and `end` act as an index of the characters that make up the `String`. The start..end syntax is a range that begins at start and continues up to, but not including, end. We can include `end` by using `..=`.

Example:

```rust
    let striker = String::from("Morata");
    let striker2 = &striker[0..=3]; // Mora
    let striker2 = &striker[..=3]; // Mora
    let striker2 = &striker[..]; // Morata
```

## Below are some general SLice principles

- Slice do not have Ownership
- A slice does not include the the character in the last index unless we add an equal sign.
- Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.
- String (`&str`) literals are slices (`let s = "Rustlings!"`).
- Slices also apply to `arrays`.
