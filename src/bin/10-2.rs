pub trait Summary {
    // have default implication
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Display {
    fn display(&self) -> String {
        String::from("display")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("this is display")
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("summary: {}", article.summarize());
    summarize(&article);
}

// trait as the parameter
fn summarize(item: &(impl Summary + Display)) {
    println!("summary: {} {}", item.summarize(), item.display());
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}