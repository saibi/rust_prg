mod aggregator;

use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("john"),
        content: String::from("I am a tweet"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Breaking news"),
        location: String::from("Nigeria"),
        author: String::from("John Doe"),
        content: String::from("This is a breaking news"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article: {}", article.summarize());

    let number = 5;
    println!("Number: {}", number.summarize());
}
