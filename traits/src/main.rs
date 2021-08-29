use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("beilunyang"),
        content: "Hello RUST".to_string(),
        reply: true,
        retweet: true,
    };

    println!("The tweet's summary is {}", tweet.summarize());

    let news = NewsArticle {
        content: "NewsArticle for Rustlang".to_string(),
        headline: "NewsArticle".to_string(),
        author: "beilun".to_string(),
        location: "ShangHai".to_string(),
    };

    println!("The News's summary is {}", news.summarize());

    // 为实现了Display trait的类型实现了Summary trait
    // String类型实现了Display trait, 所以拥有Summary trait的方法
    let str = String::from("Hello");
    println!("{}", str.summarize());
    println!("{}", str.summarize_author());
}
