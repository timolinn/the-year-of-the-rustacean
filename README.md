# The year of the Rustacean 🦀

This repository contains code samples and some general principles that guides major concepts in the Rust Programming Language.

Feel free to use as your refresher for certain rust concepts that can be hard to grasp. This is my first low level programming language, it hasn't been easy plus Rust is a really different animal. This short summary is heavily influenced by the Rust Book and Chris Krycho's [podcast](https://newrustacean.com).

> If you find anything wrong, typos, bugs and what not, send a pull request.

Also, remember to *star* this repo if you found it any useful 😃.

## Content

- [Borrowing and References](#borrowing-and-references-in-rust)
- [Ownership](#ownership-in-rust)
- [Slice](#the-slice-type)
- [Structs](#structs)
- [Tuple Structs](#tuple-structs)

## Borrowing and References in Rust

Every value/data has only one owner in Rust. By default, variables are immutable and block scoped (every variable becomes invalid at the end of it's scope. A Scope or block is basically a `{..}`), Rust uses _borrowing_ and _referencing_ to deal with data ownership, as opposed to _moving_ data about and having to explicitly return or pass ownership to another block or function for further use.

### Some general principles

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid
- You’re not allowed to modify something that another thing has reference to.
- You can only have one mutable reference to a particular piece of data in a particular scope.
- We also cannot have a mutable reference while we have an immutable one. In other words, we cannot borrow a mutable variable that is also borrowed as immutable.

## Ownership in Rust

- Ownership is one of Rust concepts that helps guarantee memory safety without a GC (Garbage Collector). It prevents Rust from having common low level language errors like dangling pointers, segfaults, data races etc. Ownership is Rust's central feature, it basically means that every value has an owner, as a matter of fact, only one owner at a time. *_One cannot simply understand ownership at first_*, understanding ownership is understanding most of Rust.

Take for example:

```rust
    {// name is invalid here
        let name = "Timolinn"; // name is valid here
                             // still valid here
    } // name is out of scope, so it's now invalid.
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

A _string slice_ is a reference to a part of a `String`. it is usually created with the `&[start..end]` syntax. The resulting string is a reference to a part of the original string. The `&str` type or in English _string literal_ can also be referred to as a string slice. Refer to [String](https://github.com/timolinn/the-year-of-the-rustacean#strings) for more about strings.

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

### Some general Slice principles

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

## Enums

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

The `match` operator is a control flow operator that enables you to compare a value against a series of patterns and executes code as per the pattern matched.

Quoting from the rust book:
> Think of a `match` expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a `match`, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

For example:

```rust
    enum CryptoCurrencies {
        BitCoin,
        Ethereum,
        LiteCoin,
        BitCoinCash
    }

    fn value_balance(crypto: CryptoCurrencies) -> f32 {
        match coin {
            CryptoCurrencies::BitCoin => {
                println!("Satoshi's pride!");
                1
            },
            CryptoCurrencies::Ethereum => 0.8,
            CryptoCurrencies::LiteCoin => 0.5,
            CryptoCurrencies::BitCoinCach => 0.2,
        }
    }
```

### Matching with `Option<T>` enum

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

### Some general Match Principles

- Matches are exhaustive, you must cover every possible match case to get Rust to compile your code.
- The `_` is used as default match arm, should the provided value does not match any patter.
- The match expression compares the resulting value against the pattern of each arm, in order.
- Curly brackets are allowed when defining associated code of an arm.

### if let

According to the Rust Book, The `if let` let's you combine `if` and `let` into less verbose way to handle values that match one pattern while ignoring the rest.

```rust
   if let Some(3) = some_u8_value {
        println!("three");
    }
```

## Vectors `Vec<T>`

A `vector` is a data structure that can store multiple data of the same type, it is one of rust's collections. To create a new and empty vector we can use the conventional `new()` method or the `vec!` macro.

```rust
    // note that the type annotation is important in this case because we are creating an empty vector
    let v: <i32> = Vec::new();
    let n = vec![1, 2, 3];
```

We can add elements into a vector using the `.push` method.

```rust
    let mut v: <i32> = Vec::new();
    v.push(1);
    v.push(4);
```

We can read from a vector using the index signature or the `.get` method.

```rust
    // using the index syntax
    let n = vec![1, 2, 3];
    let one = n[0];

    // using .get
    match n.get(3) {
        Some(3) => println!("matched letter {}", n.get(3)),
        _ => None
    }
```

when we need to store elements of a different type in a vector, we can define and use an enum!

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

### Some general `Vector` principles:

- They can contain only same type of data.
- When a `vector` gets dropped, all it's elements are dropped too.
- You can add tot a vector using the `.push` method.
- You can read from a vector using the index syntax or the `.get` method.
- When you try to access an invalid index in a `vecror`, the `.get` (`v.get(9_000)`) method returns `None` without panicking while Rust will panick if you perform the same operation using the index syntax (`&v[9_000]`).
- Rust will not let you extend a mutable vector if there's an immutable reference to the same `vector` already. Check ownership rules above.
- You can iterate over elements in a `vector`, mutably or immutably using loops.
- Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.

## Strings

`String` type is a wrapper over a `Vec<u8>`. It is `utf-8` encoded and is allocated on the heap. There are 3 relevant ways to look at `String` type in Rust:

- as bytes
- as scalar values
- as graheme clusters

Rust has only one string type in the core language, which is the string slice `str`. The `String` type is made available by the Rust `std` library, it is growable and can be mutated.
To create a new `String` we can do the following:

```rust
    let mut s = String::new();
    s.push_str("Hello World~");

    let ns = String::from("Hello world");

    let an_s = "hello world".to_string();

    // Rust String are utf-8 encoded so we can also do the following.
    let hello = String::from("नमस्ते");
    let hello = String::from("你好");
```

Like every other proramming language, there is string concatenation in Rust. Generally there are two ways to string concatenate in Rust:

```rust
    // using the `+` operator
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    // the hello value wi be moved here
    let hello_world = hello + &world; // the hello variable is dropped here.

    // OR using the `format!` macro.
    let s1 = String::from("are you");
    let s2 = String::from("a");
    let s3 = String::from("Rustacean");
    let s = format!("Hey! {} {} {}?", s1, s2, s3);
```

### `String` Slicing.

Rust's `char` type can vary in size, it is based on the string encoding.
For example:

```rust
    let s = "hello"; // 5 bytes
    let slice_s = &s[0..1]; // h

    let s2 = "Здравствуйте"; // 24 bytes
    let slice_s2 = &s2[0..1]; // PANIC!!!
```

Because of this reason we must create string slices with caution because it can crash our program if we ever try to access
the wrong range.
Notice `let slice_s = &s[0..1];` returns only `h`? We can ask rust to include the `char` of the index `1` by
slicing like this:

```rust
    let slice_s = &s[0..=1]; // he
```

### Iterating over a String.

Fortuantely we can manipulate individual `char`s of a `String` by interating over them. Study and run the examples below:

```rust
    for c in "नमस्ते".chars() {
       println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
```

Notice the `.bytes()` and `.chars()` methods?

### Some general `String` principles

- Rust has only one string type in it's core, that is the `str` usually seen as `&str` in most codebases.
- String literals are stored in the binary output of a program and are known as string slices.
- There are three ways to look at a `String` in Rust's perspective, `bytes`, `scalar values` and `grapheme clusters`.
- String indexing is not allowed in Rust for many reasons, one of which is that determining the perspective of which the
programmer is accessing the `String` from is not trivial, due to Rust's way of encoding strings.
- Under the hood Rust's `String` type is a `Vec<u8>`.
- The `format!` macro works like the `println!` macro but returns the string instead printing to stdout.
- You can use the `+` or the `format!` macro to concatenate a string.
- We can iterate over strings as chars or as bytes.

## HashMap

`HashMap<K, V>` stores a mapping of keys of type K to values of type V. It does this via a hashing function, which determines how it places these keys and values into memory. It's like an associative array for the PHP developer or the ES6 Map for Javascript developer.

To create a `HashMap` we must first import the type.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // use .insert to add key value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

You can read value from a `HashMap` using the `.get(k: K)` method.

```rust
    let team_blue = String::from("Blue");
    let team_blue_score = score.get(&team_blue); // This will return a `Some(v)`
```

<b>[Read more about hashmaps here.](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)</b>

### Some general `HashMap` principles

- `HashMaps` store data on the heap.
- `HashMaps` are homogeneous, ie all keys must be of the same type, same goes for all the values.
- Rust can infer types within a hashmap.
- Types that implement the `Copy` trait are copied into the `HashMap`, while owned types are moved in to the `HashMap`.

```rust
    use std::collections::HashMap;

    let color = String::from("Favorite color");

    let mut map = HashMap::new();
    map.insert(color);
    // color is invalid at this point.
```

- You can pass references into a HasMap and the type will not be moved, however we must use lifetimes to
make it work.
- You create a `HashMap<K, V>` using the conventional `new()` method: `let map = HashMap::new();`
- You can also create a new `HashMap` from an array of tuples by calling the `.collect()` method on it.

```rust
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

- Use the `.insert(K,V)` method to add a value to a `HashMap<K, V>` and `.get(&K)` to retrieve value.
- For updating existing `HashMap` read [here](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-hash-map).

## Error Handling

Errors are a fact of life in software development, Rust has a number of ways for handling "errorable" situations. I must mention that Rust does a lot work in trying to standardize error reporting by providing multiple features for handling errors. Rust sometimes takes care of the error handling for you if you want.

One way it does this is by using the `?`, another way is by calling `unwrap()` method on a `Result` type. Both the `?` and `unwrap()` method tells rust to go ahead and eveluate the returned types by returning an `Ok(V)` when the operation was successful, by `panicking` or propagating an error when the `Err()` type is returned. Find examples below.

Errors are grouped into two in Rust, _recoverable_ and _unrecoverable errors_. Rust does not have exceptions, instead it provides the `Result<T, E>` type for _recoverable errors_ and the `panic!` macro for crashing a program when it encounters _unrecoverable errors_.

Examples:

```rust
    fn main() {
        panic!("crash and burn 🔥🔥🔥🔥!"); // stops the program with a message and stacktrace
    }
```

Rust handle Recoverable errors with the `Result` enum

```rust
    enum Result<T, E> {
        Ok(T), // T represents the type of value to be returned in a success case
        Err(E),// E represents the type of error on failure
    }
```

Most functions in Rust `std` and in third party packages return the `Result` type if the function can `fail`.

```rust
    use std::fs::File;

    fn main() {
        // you could do this
        let f = File::open("hello.txt").unwrap();

        // OR this
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error)
            },
        };
    }

    // OR this
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // `.expect` calls the `panic` for you with the error message provided as an argument.

    // OR this
    let mut f = File::open("hello.txt")?; // notice the question mark 🙂
```

### Some general error handling priciples

- Once you learn how to work with the `Result` type, you'll have easier time propagating or handling errors in Rust.
- The `Result` enum has two variants `Ok(V)` and `Err(E)`.
- You can propagate your errors to the calling function by returning the `Result` type.
- When your code `panics` it's an _unrecoverable error_, however you can handle _recoverable errors_ using the `Result<V, E>` type.
- `expect()` and `unwrap()` methods are better for prototyping and tests. You may use them as placeholders to get your protoype up and running until you're ready to make your code more robust, you can come back use the `Result`.
- Returning `Result` in a function in your library gives the code caller an option to handle the returned error in a way that fits it's use case/scenario.
- [Read here for more error handling guideline](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling)

## Generic Types

`Generics` are a tool in Rust for effectively handling code duplication. It is an _abstract stand in_ for concrete types or properties.
That is to say that they are types that can become or enable code perform operations on abstract types. We can use generics to define function signatures or structs that we can use with many different concrete data types.

Take for example:

```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        // The compiler interpretes T as a integer here
        let integer = Point { x: 5, y: 10 };
        // The compiler interpretes T as a float here
        let float = Point { x: 1.0, y: 4.0 };
    }
```

Notice as T becomes whatever type we pass when creating an instance of struct `Point<T>`.

We can also define generics in struct methods:

```rust
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: "world" };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
```

Let's demostrate how generics can remove code duplication.

```rust
    // returns the largest integer in a vector of i32
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // returns the largest char in a vector of char
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
```

We can easily replace the two functions above with one, by using `generics`, thereby eliminating code duplication (think DRY).

```rust
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                    largest = item;
                }
            }

        largest
    }
```

### Some general `generics` principles

- `Generics` help minimize duplicate code.
- Combining `generics` and `trait` bounds is an idiomatic way of defining desired behaviour for your functions, methods, structs, traits etc.
- Using generics does not make your code any slower than using concrete types.
- _*Monomorphization*_ is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

## Traits

A trait tells the Rust compiler about functionality a particular type has and can share with other types. Traits are similar to a feature often called interfaces in other languages, although with some differences one of which is that `traits` in Rust can have default implementations.

We can use _trait bounds_ to specify that a generic can be any type that has certain behavior. A type’s behavior consists of the methods we can call on that type.

```rust
    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize_author(&self) -> String;

        fn author(&self) -> String {
            format!("Written by {:?}", self.summarize_author())
        }
    }
```

The `Summary` trait has three methods, `summarize`, `summarize_author` and `author`. The `author` method has a default implementation this means that any type that implements this trait has access to the the deafault method.

Implementing a trait on a type:

```rust
    pub struct Tweet {
        pub message: String,
        pub author: String
    }

    pub struct NewsArticle {
        pub headline: String,
        pub author: String
    }

    pub struct LincolnsLetter {
        pub subject: String,
        pub author: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("The lad tweeted: {}", self.message)
        }

        fn summarize_author(&self) -> String {
            format!("@{:?}", self.author)
        }
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}!!!", self.headline)
        }

        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    impl Summary for LincolnsLetter {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }
```

Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name that we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.

### Trait Bound Syntax

```rust
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
```
The notify function takes a generic type parameter of `T` that must implement the `Summary` trait.
We can also use the `+` operator and the `where` keyword to define clearer and more concise trait bounds.

```rust
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

The code above is same as the code below but made more clearer with the where keyword:

```rust
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
```

### Some general `trait` principles

- One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate. it means we can’t implement external traits on external types.
- Traits are similar to a feature often called interfaces in other languages, although with some differences one of which is that `traits` in Rust can have default implementations.
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
- The Summary trait would also need to be a public trait for another crate to implement it.
- Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
- Use the `where` keyword for a clearer implementation.
- To return a type that implements some trait we do `fn some_function() -> impl SomeTrait`.
- We can also conditionaly implement methods based on trait bounds. [More here](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)

## Lifetimes

Every `reference` in Rust has a `lifetime`. `Lifetimes` specify how long a reference should live before it gets `dropped`.
Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### Lifetimes in `Structs`

It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition

Example from the Rust book:

```rust
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }
```

In the example above, an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.


### Lifetime Elision



### Some general `lifetimes` priciples

- `Lifetimes` are mostly inferred, just like `types`. Same way just like `types`, `lifetimes` must be _annotated_ when multiple `lifetimes` are possible.
- When we specify the lifetime parameters in a function signature, we’re not changing the `lifetimes` of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to the constraints.
- The smaller of the `lifetimes` specified in a function definition becomes the main constraint in determining if the references will still be valid at the end of the function scope.
- When returning a reference from a function, the `lifetime` parameter for the return type needs to match the `lifetime` parameter for one of the parameters, or to a value created within the function, which would be a dangling reference because the value will go out of scope at the end of the function.
- The patterns programmed into Rust’s analysis of references are called the _lifetime elision rules_.
- Lifetimes on function or method parameters are called _input lifetimes_, and lifetimes on return values are called _output lifetimes._
- The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations, they include
    + Each parameter gets its own lifetime.
    + If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    + If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters

    The compiler throws an error if all three laws fail.

### Closures

- Closures are also known as anonymous functions. They are functions defined like variables and can also be passed as arguments to other functions. Unlike functions they have the ability to capture values from the scope of which they are defined

```rust
    let a = 2;
    let b = 4;
    let calc = |a, b| {
        a + b
    };
```

### Some general `closure` principles

- `Closures` can capture values from the scope they were defined.
- They can be defined and assigned to a variable and executed in a different context.
- They do not require type anotations of parameters and return types.
- `Closures` may implement the following 3 function traits provided by the rust standard library:
  - `FnOnce`: This means the closure takes ownership of the variables from it's scope and consumes them, thereby they can only be called once. This can be forced with the `move` keywork:

    ```rust
        let x = 3;
        let y = 40;
        let product = move |x, y| x * y;
    ```

  - `FnMut`: This means the closure borrows the variables mutably
  - `Fn`: This means the closure borrows the variables immutably

## Smart Pointers

Smart pointers originated from C++, they are like references (`&`) but provide more capabilities and contain metadata. Some smart pointers in rust that we've already used are the `String` and `Vec<T>` types, both own some data and provide some metadata and capabilities on them.
The following are other smart pointers in rust:

- `Box<T>` - for allocating data on the heap.
- `Rc<T>` - for creating multiple ownerships of data
- `Ref<T>` and `RefMut<T>` - accessed through the `RefCell<T>`, it enforces the borrowing and ownership rules of Rust at runtime.

### Some general `smart pointers` principles

- The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references.
- When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.
- Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion converts a reference to a type that implements Deref into a reference to a type that Deref can convert the original type into.

  ```rust
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        fn main() {
            let m = MyBox::new(String::from("Rust"));
            hello(&m);
        }
  ```

  In the code above we don't need to explicitly add the deref operatpr (`*`) because of the implicit deref coercion feature of rust.
- The `Drop` trait lets you customize what happens when a value goes out of scope. This is basically a function called `drop` that the Rust calls automatically when the value is out of scope. The compiler inserts the call to this method where needed.

## Concurrency

Concurrency is when different parts of a program execute independently, while parallellism is when different parts of a program run at the same time. When we say concurrency think Processes, Threads. A process contains 1 or more threads, the process manages the resources available to the threads. A thread is a sequence of instructions that is to be executed by the operating system.

Concurrency can improve the performance of our programs, but they complexity in them. When multiple threads are running in the same memory space, sharing the same resource available in a process this can lead some known problems in our programs;

- Race Condition: Multiple threads trying to access the same memory
- Deadlocks: Two threads waiting on each other to finish using a resource one of them is supposedly using, preventing both threads from continuing.
- Hard to reproduce circumstance that led to a bug.

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
```

Note that the `handle.join().unwrap()` call ensures that the main thread waits for the spawnwd thread to complete execution before exiting.

*Channels* are means for communication between threads. Threads or Actors communication by sending each other messages containing data through channels.

```rust
    use std::sync::mpsc;
    use std::thread;

    pub fn main() {
        let (tx, rx)= mpsc::channel();

        thread::spawn(move || {
            let value = 50;
            tx.send(value).unwrap();
        });

        let recvd = rx.recv().unwrap();
        println!("Got: {}", recvd);
    }
```

The Rust compiler won't compile if the data types that would pass through the channel is not defined, in the example above, rust infers the data types through the usage in the spawned thread.

*Shared State Concurrency* is another way of handling concurrency apart from using channels. Rust is well equipped to use shared state concurrency effectively compared to many other programming languages. We can effectively share memory with Mutexes which is so tricky that most developers prefer to use channels for concurrency, however the combination of Rust's type system, smart pointers and ownershiop rules, we cannot get locking nad unlocking with mutexes wrong.

```rust
    use std::sync::Mutex;

    pub fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 9;
        }

        println!("{:?}", m);
    }
```

Combination of mutexes and smart pointers enable us write programs that uses multi threading and `multiple ownerships`. This can be achieved by using the `Arc<T>` from the standard library.

```rust

    use std::sync::{Mutex, Arc};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for idx in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread {} spawned", idx);
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
```

While the `Arc<T>` and mutex locks enable us to have multiple ownerships between threads, we can as well use the `std::marker::Send` trait to transfer ownership between threads. Also the `std::marker::Sync` indicates that a reference to `T` (`&T`) can be sent saftely to another thread.

### Some general `Concurrency` principles in Rust

- Rust has much lower level control over operating system threads as opposed to `green threads` in Golang.
- A channel in programming has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you a rubber ducks into a river, and the receiver half is where the rubber duck ends up downstream.
- A channel is a means of communication between threads. *Do not communicate by sharing memory, share memory by communicating* - Golang Slogan
- The `channel` takes ownership of the sent variable.
- The Rust std implementation of channels allow only one `receiver` but can allow multiple `producers` or `senders`
- Shared-State Concurrency is another way of handling concurrency, it simply means `sharing memory` in the sense that data/value transferred through a channel maybe accessed by multiple threads which leads to multiple ownership. Rust is very much equipped to handle this type of concurrency with it's type system, smart pointers and ownership rules.
- Shared state concurrency requires the use of `Mutexes` (_mutual exclusion_). This is the process of `guarding` the data by the mutex. Every thread is required to request for access before accessing the data from the mutex's lock.
- Using mutexes can be incredibly tricky, however, thanks to Rust's type system you cannot get locking and unlocking wrong.
- `Mutex<T>` provide interior mutability and can be used to mutate contents inside an `Arc<T>`, same way the `Cell` family does. Specifically, we can provide interior mutability in a `Rc<T>` using the `RefCell<T>`.
- `Arc<T>` provides multiple ownership functionality in a thread safe way as opposed to `Rc<T>` it is _atomic reference counted type_.
- `Mutex<T>` comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.
