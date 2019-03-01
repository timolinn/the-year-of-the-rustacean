fn main() {
    let s = "new string slice";
    let s_collection = "new string slice".to_string();

    // or
    let mut sm = String::new();
    sm.push_str("foobar");

    let an_s = String::from("s: Cow<'a, str>");

    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("01");
    let s2 = String::from("29");
    let s3 = String::from("00");
    let s = format!("The time is {}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
