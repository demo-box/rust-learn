// User派生自Debug, 实现打印相关的trait
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 结构体方法声明
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数声明
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 可以有多个impl
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.width > other.height
    }
}

fn main() {
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 10,
    };

    // 整个结构体实例可以标记为可变的，但不能将结构体实例中的某个字段标记为可变
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 10,
    };

    user2.email = String::from("anotherone@example.com");

    let user3 = build_user(String::from("someone@example.com"), String::from("someone"));
    println!("user3 is {:#?}", user3);

    // 使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例
    // 可以通过结构体更新语法实现
    let user4 = User {
        email: String::from("anotherone@example.com"),
        username: String::from("anotherone"),
        ..user1 // ..语法指定了剩余未显示设置值的字段应与给定实例对应字段相同
    };
    println!("user4 is {:#?}", user4);

    // 定义元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // black与origin的类型是不同的
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 结构体示例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!(
        "The area of the rectangle is {:?} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // 关联函数调用::
    let sq = Rectangle::square(3);
}

// 结构体可以作为函数的返回值
fn build_user(email: String, username: String) -> User {
    /*
     * 变量名和字段名相同时，可以简写
     *  User {
     *      email,
     *      username,
     *      active: true,
     *      sign_in_count: 10,
     *  }
     */
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 10,
    }
}
