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
- Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index` like here `&[starting_index..ending_index]`.
- String (`&str`) literals are slices (`let s = "Rustlings!"`).
- Slices also apply to `arrays`.

## Structs

Structs are like tuples but more flexible, in that each piece of data can be named in such a way that it is clear what the value means.

```rust
    struct User {
        name: String,
        email: String,
        age: u32,
        sex: String,
        active: bool,
    }
```

Struct definition is like a general template for a type, instances fill in that template with particular data to create values of the type. An example of an instance is below:

```rust
    let MrBean = User {
        email: String::from("johnenglish@mi7.com"),
        name: String::from("Johnny English"),
        age: 18,
        sex: String::from("Male"),
        active: true
    };
```

### Below are some general `Struct` principles

- We can use the dot notation to get a specific value _eg._ use `MrBean.age` to get the value of age.
- We can't change the value of any data unless the instance is mutable, `let mut MrBean = User {...}`.
- Marking only certain fields as mutable is not allowed by the rust compiler.
- Rust's field init shorthand syntax let's us define a value with a variable of the same name without having to write the variable name twice. So `User { name: name }` is same as `User { name }`
- `impl` blocks (implementation block) hold methods that specify the behavior that instances of the struct possess.
- `structs` are allowed to have multiple `impl` blocks.
- `derive` annotation can be used to add useful behaviors to structs by implementing `traits` automatically.
- Rust's Struct update syntax let's create new instances based off the data of a previous instance. _eg._

```rust
    let mut MrsBean = User {
        email: String::from("mrsjohnenglish@mi7.com"),
        name: String::from("Sandra English"),
        sex: String::from("Female"),
        age: MrBean.age,
        active: MrBean.active // update syntax
    };
```

OR

```rust
    let mut MrsBean = User {
        email: String::from("mrsjohnenglish@mi7.com"),
        name: String::from("Sandra English"),
        sex: String::from("Female"),
        ..MrBean
    };
```

### Tuple Structs

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.

```rust
    struct Color(i32, i32, i32);
    struct Coord(i64, i64);

    let black = Color(0, 0, 0);
    let naija = Coord(4, 14);
```

Tuple structs still behaves like ordinary tuples, you can destructure them, you can also use a `.` notation followed by the index to access an individual value.