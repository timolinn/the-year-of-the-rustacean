# 100DaysOfRustLang 🔥🦀🔥

This repository contains code samples and some writings based on my understandings of the Rust Programming Language. Feel free to use as your refresher for certain rust concepts that can be hard to grasp. This is my first low level programming language, it hasn't been easy plus Rust is a really different animal. Some of the text here is gotten (mostly NOT verbatim) the Book, Chris Krycho's [podcast](https://newrustacean.com).

> If you find anything wrong, typos and what not, I'd be glad to to receive your pull request 😃.

## Borrowing and References in Rust

Rust uses borrowing and referencing to deal with variables/data ownership, as opposed to _moving_ data about and having to explicitly return ownership for further use.

### Rules of References

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid
- We’re not allowed to modify something we have a reference to.
- You can only have one mutable reference to a particular piece of data in a particular scope.
- We also cannot have a mutable reference while we have an immutable one. In other words, we cannot borrow a mutable variable that is also borrowed as immutable.

## Ownership in Rust

- Ownership is one of Rust concepts that helps guarantee memory safety without a GC (Garbage Collector). It prevents Rust from having common low level language errors like dangling pointers, segfaults etc. Ownership is Rust's central feature, it basically means that every value has an owner, as a matter of fact, only one owner at a time.

Take for example:

```rust
    {// me is invalid here
        let me = "Timolinn"; // me is valid here
                             // still valid here
    } // me is out of scope, so it's now invalid.
```



### Some general Ownership principles

- Rust use the notion of `borrowing` and `moving` to implement ownership rules.
- Ownership  rules are checked at compilation level using a tool called the `borrow checker`.
- Data can be `borrowed` or `moved`, when you `borrow` data you get a reference to that value.
- Each value in Rust has the `owner`.
- No two scopes can `own` the same data/variable, You can have as many immutable references as you want, but you can only have one `mutable` reference at a time.
- You can't have another reference to a `mutable value`, even it is an `immutable reference`.
- When the owner goes out of scope, the value will be `dropped`.
- Moving large chunks of data can be pretty expensive, use References.
- Rust data values use scoped to determine validity, unless `borrowed` or `moved` the value is dropped at the end of it's scope/block.
- Every value remains valid until it goes out of scope.
- In Rust memory belongs to `scope`.

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

### Some general SLice principles

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

### Some general `Struct` principles

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

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Useful for scenarios where you want named tuples, much like python's `collections.namedtuple` that creates tuple-like objects.

```rust
    struct Color(i32, i32, i32);
    struct Coord(i64, i64);

    let black = Color(0, 0, 0);
    let naija = Coord(4, 14);
```

Tuple structs still behaves like ordinary tuples, you can use them with the update syntax, you can also use a `.` notation followed by the index to access an individual value.

```rust
    println!("{}", black.0); // 0
    println!("{}", naija.1); // 14
```

## Enumns

Enumerations or Enums as mostly referred to allows the definition of types by enumerating through a possible set of values.
Enums are like variants, no data can be of more than one variant of the same enum. Classic example from the Rust book
is the two IP address types (IPV4 and IPV6), the two fundamental types of Ip Addresses. Each Enum variant can be used as a type.

```rust
    enum IpAddr {
        V4,
        V6
    }
```

The code above create an enum witht two variants, `v4` and `V6`. Therefore we can create instances of each type of IP Address like
this:

```rust
    let ipv4 = IpAddr::V4;
    let ipv6 = IpAddr::V6;
```

ALternatively, we can use structs to improve the code:

```rust
    struct Ipv4Addr {
        addr: String,
    }

    struct Ipv6Addr {
        addr: String,
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
```

Here we use structs to store our data before wrapping them into our enum. This can also be achieved without
using structs. See the example below:

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let localhost = IpAddr::V4(String::from("127.0.0.1"));
```

### The `Option` enum.
The `Option` enum is defined by the Rust standard library,
it encodes the very common scenarios where a value could be something or nothing. Null does not exist in Rust, the `Option` enum is Rust's way of expressing a null or not-null scenarios.

```rust
    enum Option<T> {
        Some(T),
        None
    }
```

The `Option<T>` enum has two variants `Some(T)` and `None`. The symbol `<T>` defines a generic type `T`, this means that `Some(T)` can contain data/value of any type.

```rust
    let some_string = Some("Rustacean");
    let some_number = Some(12);

    let absent_number: Option<i32> = None;
```

Notice we didn't have to import `Option<T>` or `Some()` or `None`? This is beacuse by defaults they are included in the prelude, that's enough to note how important they are to the language.

### Some general `Enum` principles
- Enums can encapsulate multiples types or variants.
- A variant van be of any type.
- We can pass data directly inside an enum instead of using structs.
- Enums can have `impl` blocks.
- The `Option` enum can be be `Some` or `None`, swap those with `NotNull` or `Null` respectively, incase you need more explanation.
- The `Option` enum, `Some` and `None` values are included in the prelude, therefore you don't need to import `Option` to use
it or namespace `Option::Some()` to use it.
- `Null` values does not exist in Rust 😎, the `Option` enum is here to save the day 😄.
- In Rust, `Nullable` values are not assumed.
- To extract the `v` in `Some(v)` use pattern match or just `let v = Some(v).unwrap();`.

## Pattern Matching