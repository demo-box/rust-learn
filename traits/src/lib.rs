use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Trait下的方法可以有默认实现, 并且默认实现中可以使用Trait中还未实现的方法
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

// impl trait语法
pub fn notify(item: impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// trait bound
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

// 使用 + 指定多个trait
pub fn subscribe(item: impl Summary + Display) {
    println!("Breaking news: {}", item.summarize());
}

pub fn subscribe2<T: Summary + Display>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

// 和listen2一样
pub fn listen<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news: {}", a.summarize())
}
// 使用where子句
pub fn listen2<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news: {}", a.summarize())
}

// impl trait语法，作为返回值类型约束
pub fn watch(flag: bool) -> impl Summary {
    // 会报错，虽然NewsArticle和Tweet都实现了Summary trait, 但属于两种不同的类型，只能返回一种确定的类型
    // if (flag) {
    //     return NewsArticle {
    //         content: "NewsArticle for Rustlang".to_string(),
    //         headline: "NewsArticle".to_string(),
    //         author: "beilun".to_string(),
    //         location: "ShangHai".to_string(),
    //     };
    // }

    Tweet {
        username: "beilun".to_string(),
        content: "tweet content".to_string(),
        reply: true,
        retweet: true,
    }
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 为所有实现了Display trait的类型，实现Summary trait
impl<T: Display> Summary for T {
    fn summarize_author(&self) -> String {
        String::from("Beilun")
    }
}
