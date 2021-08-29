// 函数中使用泛型
// fn largest<T>(list: &[T]) -> T {
//     list[0]
// }

// struct中使用泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 在方法中使用泛型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 为特定的i32实现y方法，只有当Point中的T为i32时，才可以使用y方法
impl Point<i32, i32> {
    fn y(&self) -> &i32 {
        &self.x
    }
}

// enum中使用泛型
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = Point { x: "hello", y: 'c' };
    let p4 = p3.mixup(p1);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
}
