// use std::fmt::Result;
// use std::io::Result as ioResult; // 通过as关键字可以设置别名
// 相当于
// use std::{fmt, io};

// use std::io;
// use std::io::Write;
// 相当于
// use std::io::{self, Write};

// 使用通配符*将collections中的所有内容都引入进来
// 注意:谨慎使用
// 应用场景: 测试。将所有被测试代码引入到tests模块
// use std::collections::*;

fn f1() -> Result {}

fn f2() -> ioResult {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用pub关键字，可以将use的模块对外暴露出来
pub use crate::front_of_house::hosting;

fn main() {
    println!("Hello, world!");
}
