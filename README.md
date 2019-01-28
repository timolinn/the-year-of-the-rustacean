## Borrowing and References in RUST

Rust uses borrowing and referencing to deal with variables/data ownership, as opposed to _moving_ data about and having to explicitly return ownership for further use.

### Rules of References

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid
- We’re not allowed to modify something we have a reference to.
- You can only have one mutable reference to a particular piece of data in a particular scope.
- We also cannot have a mutable reference while we have an immutable one. In other words, we cannot borrow a mutable variable that is also borrowed as immutable.

## Ownership in Rust

## The Slice Type

A _string slice_ is a reference to a part of a `String`. it is usually created with the `&[start..end]` syntax. The resulting string is a reference to a part of the original string.

For example:

```rust
    let name = String::from("Timothy"); // Timothy
    let nameSlice = &[0..3]; // Tim
    let nameSlice2 = &[0..4]; // Timo
```

The `start` and `end` act as an index of the characters that make up the `String`. The `start..end` syntax is a range that begins at start and continues up to, but not including, `end`. We can include `end` by using `..=`.

Example:

```rust
    let striker = String::from("Morata");
    let striker2 = &striker[0..=3]; // Mora
    let striker2 = &striker[..=3]; // Mora
    let striker2 = &striker[..]; // Morata
```

### Below are some general SLice principles

- Slice do not have Ownership
- A slice does not include the the character in the last index unless we add an equal sign.
- Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.
- String (`&str`) literals are slices (`let s = "Rustlings!"`).
- Slices also apply to `arrays`.
