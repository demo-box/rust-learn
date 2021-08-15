use core::panic;
use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let username = read_username_from_file().unwrap();
    println!("username:{}", username);

    let f = File::open("hello.txt");

    let f1 = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening file {:?}", err);
        }
    };
    // unwrap和expect都相当于上面的match代码
    // unwrap无法自定义错误信息,expect可以
    // let _f = File::open("hello.txt").unwrap();
    // let _f = File::open("hello.txt").expect("无法打开相应文件");

    let ff = File::open("hello.txt");

    let f2 = match ff {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };
}
