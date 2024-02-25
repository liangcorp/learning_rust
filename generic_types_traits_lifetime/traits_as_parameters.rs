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

    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }


}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
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