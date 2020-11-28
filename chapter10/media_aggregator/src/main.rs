use media_aggregator::{self, notify, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win"),
        location: String::from("Pttsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburfgh Penguins once again are the best 
            hockey team in the NHL.",
        ),
    };
    println!("New articl earrive: {}", article.summarize());

    notify(&tweet);
    notify(&article);
}
