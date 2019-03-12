use std::fmt::Display;

pub trait Summary {
    fn summary(&self) -> String {
        String::from("You can read the summary in his book")
    }

    fn summarize_author(&self) -> String;

    fn author(&self) -> String {
        format!("Written by {:?}", self.summarize_author())
    }
}

pub trait Show {

}


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
    fn summary(&self) -> String {
        format!("The lad tweeted: {}", self.message)
    }

    fn summarize_author(&self) -> String {
        format!("@{:?}", self.author)
    }
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
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

impl Show for NewsArticle {}

// This is a syntax sugar for notify2()
pub fn notify1(item: impl Summary + Show) -> () {
    println!("{:?}", item.summary());
}

// Trait bounds!
pub fn notify2<T: Summary + Show>(item: T) -> () {
    println!("{:?}", item.summary());
}

/// Still trait bounds
/// Means T must implement Summary and Show Traits
/// Means U must implement Show trait
pub fn notify3<T, U>(item: T, item2: U) -> ()
    where T: Summary + Show,
          U: Show
{
    println!("{:?}", item.summary());
}

fn new_tweet(message: String, author: String) -> impl Summary {
    Tweet {
        message,
        author
    }
}

fn main() {
    let tweet = Tweet { message: "unpopular opinion".to_string(), author: "timolinn".to_string() };
    println!("{:?}", tweet.summary());

    let news = NewsArticle { headline: "breaking news".to_string(), author: "The guardian".to_string() };
    println!("{:?}", news.summary());

    let letter = LincolnsLetter { subject: "For the sake of the union.".to_string(), author: String::from("Abraham Lincoln.") };
    println!("{}", letter.summary());
    println!("{}", letter.author());


    notify1(news);
}
