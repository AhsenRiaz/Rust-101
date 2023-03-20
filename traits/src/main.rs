pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", &self.headline, &self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", &self.username, &self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("Bob"),
        content: String::from("This is my article"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Alice"),
        headline: String::from("Laptop"),
        content: String::from("A good laptop in reasonable price"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Tweet summary: {}", article.summarize());
}
