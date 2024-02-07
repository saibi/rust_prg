mod aggregator;

use aggregator::*;

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

    notify(&number);
    notify(&tweet);

    notify2(&number);

    notify3(&number);
    // notify3(&article); // this will not compile because NewsArticle does not implement Display

    notify4(&tweet, &number)
}
