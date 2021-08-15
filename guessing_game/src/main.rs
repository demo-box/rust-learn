use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("开始猜数游戏!");

    loop {
        println!("输入你猜测的数字!");

        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1, 101);

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是: {}", guess);

        println!("实际数字是: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
            Ordering::Greater => println!("太大了"),
            Ordering::Less => {
                println!("太小了");
            }
        }
    }
}
