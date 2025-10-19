// -------------------------
// TRAIT DEFINITION
// -------------------------
trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("(Author info not provided)")
    }
}

// -------------------------
// STRUCT DEFINITIONS
// -------------------------
struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// -------------------------
// TRAIT IMPLEMENTATIONS
// -------------------------
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("Written by {}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    // summarize_author() will use the default implementation
}

// -------------------------
// GENERIC FUNCTION
// -------------------------

// Generic function that works with ANY type implementing Summary
fn notify<T: Summary>(item: &T) {
    println!("Breaking News: {}", item.summarize());
}

// OR you can use multiple trait bounds like this:
// fn notify<T: Summary + Display>(item: &T) { ... }  // Example if needed later

// -------------------------
// MAIN FUNCTION
// -------------------------
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust conquers the world!"),
        author: String::from("Prarthana Gade"),
        content: String::from("Rust has officially taken over the programming world."),
    };

    let tweet = Tweet {
        username: String::from("rustacean_007"),
        content: String::from("Learning Rust traits with generics today!"),
        reply: false,
        retweet: false,
    };

    // Using the trait methods directly
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());

    // Using the generic function
    notify(&article);
    notify(&tweet);
}
