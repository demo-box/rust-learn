fn main() {
    let string1 = String::from("hello");
    let string2 = "world";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("callme, some years age.");
    let first_sentence = novel.split(',').next().expect("Could not found a 'a'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first_sentence is {}", i.part);
}

// 'a 为x和y生命周期的交集
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期，结构体成员的生命周期必须比结构体长
struct ImportantExcerpt<'a> {
    part: &'a str,
}
