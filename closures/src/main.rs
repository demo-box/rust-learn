use std::{ops::IndexMut, thread, time::Duration};

// 记忆化
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cacher = Cacher::new(expensive_closure);
    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} pushups!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", cacher.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];
    // 普通函数无法捕获函数定义时的作用域内的变量
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // 使用move关键字，使闭包获得了x的所有权，所以只用x就不能再被借用了
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let equal_to_x = |z| z == x;

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y))
}
