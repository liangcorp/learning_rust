trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,
                                    self.author,
                                    self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("We won"),
        location: String::from("Australia"),
        author: String::from("Chen"),
        content: String::from("We won world cup!"),
    };

    println!("News Article\n{}", new.summarize());

    let tweet = Tweet {
        username: String::from("Chen"),
        content: String::from("We won the match"),
        reply: false,
        retweet: false,
    };

    println!("New Tweet\n{}", tweet.summarize());
}