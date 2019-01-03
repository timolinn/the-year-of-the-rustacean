# Borrowing and References in RUST

Rust uses borrowing and referencing to deal with variables/data ownership. Instead of _moving_ data about and having to explicitly return ownership for further use.

## Rules of References

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid
- We’re not allowed to modify something we have a reference to.
- You can only have one mutable reference to a particular piece of data in a particular scope.
- We also cannot have a mutable reference while we have an immutable one. In other words, we cannot borrow a mutable variable that is also borrowed as immutable.
