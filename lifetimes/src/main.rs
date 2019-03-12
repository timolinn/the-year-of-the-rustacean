use std::fmt::Display;

fn main() {
    let strng1 = String::from("Huawei");
    let strng2 = "Apple";

    let lngstStr = longest(&strng1.as_str(), &strng2);
    println!("{}", lngstStr);
}

/// Explicit lifetime annotation is required here because
/// Rust is not sure which lifetime gets assigned to the output
/// parameter between the two lifetimes of the parameters
/// So we annotate the lifetime to tell rust that we are
/// sure that the returned type will live as long as the
/// 'a lifetime.
///
/// The borrow checker will go ahead and ensure that we know
/// what we are saying and will throw an error if it finds out we lied
/// which is when the returned type doesn't live as long as the 'a lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Everything put together
/// Trait Bounds
/// Generic types
/// Lifetimes
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}