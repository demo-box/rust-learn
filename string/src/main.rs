fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello s1");
    s1.push('!');
    println!("s1:{}", s1);

    let s2 = String::from("Hello s2");
    println!("s2:{}", s2);

    let s3 = "Hello s3".to_string();
    println!("s3:{}", s3);

    let s4 = String::from("Hello s4");
    let s5 = String::from(" + Hello s5");
    let s6 = s4 + &s5; // + &s5时会发生解引用强制转换，把&string转换成&str
    println!("s6:{}", s6);
    // println!("s4:{}", s4); s4所有权发生来转移，不可再用
    println!("s5:{}", s5); // s5通过借用，所有权没有转移

    let s7 = String::from("Hello s7");
    let s8 = String::from("Hello s8");

    let s9 = format!("{}-{}-{}-{}", "Before Hello,", s7, s8, "After Hello");
    println!("s9:{}", s9);
    println!("s7:{}", s7); // 通过format!, 所有参数的所有权都不会发生转移，之后都可以继续使用
    println!("s8:{}", s8);

    let len = String::from("中文").len(); // len返回字符串的字节数
    println!("The 中文 len:{} ", len); // len是6

    let w1 = "中国语";
    // 字节
    for b in w1.bytes() {
        println!("{}", b);
    }
    // unicode标量
    for c in w1.chars() {
        println!("{}", c);
    }
    // 字形簇，标准库没有提供，需要第三方库实现, 最接近字母

    // let w2 = "这是一段中文";
    // let w3 = &w2[0..4];    // 会发生panic, 因为切割字符串需要沿char边界切割
    // println!("w3:{}", w3);
    let w2 = "这是一段中文";
    let w3 = &w2[0..3]; // index: 3是char是中国语的char边界
    println!("w3:{}", w3);
}
