use std::fmt::{self, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify_composite(item1: &impl Summary, item2: &impl Summary)
pub fn notify_composite<T>(item1: &T, item2: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "headline: {}, by {}", self.headline, self.author)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "content: {}, by {}", self.content, self.username)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {} ", self.username, self.content)
    }
}

pub fn return_summarizable(switch: bool) -> impl Summary {
    if switch {
        return Tweet {
            username: String::from("horse_ebook"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
    }

    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }

    // NewsArticle {
    //     headline: String::from("Penguins win"),
    //     location: String::from("Pttsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The pittsburfgh Penguins once again are the best
    //         hockey team in the NHL.",
    //     ),
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
