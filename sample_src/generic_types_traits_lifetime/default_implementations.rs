trait Summary {
    fn summarize(&self) -> String {
        format!("(Read..more {}...)", self.summarize_auth())
    }

    fn summarize_auth(&self) -> String;
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

}

impl Summary for Tweet {
    fn summarize_auth()(&self) -> String {
        format!("@{}", self.author)
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