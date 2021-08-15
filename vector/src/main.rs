use std::vec;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    let third = &v1[2];
    println!("The third element is {}", third);

    let v2 = vec![1, 2, 3, 4, 5];
    match v2.get(2) {
        Some(third) => print!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    // 这行代码会报错，因为同一作用域内不能既有不可变借用又有可变借用
    // v3.push(6);
    println!("The first element is {}", first);

    let v4 = vec![1, 2, 3, 4, 5];
    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![1, 2, 3, 4, 5];
    for i in &mut v5 {
        *i += 50;
    }

    for i in v5 {
        println!("{}", i)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in row {
        println!("The row's cell is {:?}", i);
    }
}
