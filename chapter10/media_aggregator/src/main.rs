use media_aggregator::{self, notify, return_summarizable, NewsArticle, Summary};

fn main() {
    let tweet = return_summarizable(false);
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

    notify(&article);
}
