// 默认约定src/main.rs是binary crate的crate root，当存在main.rs时，说明这个package中有一个binary crate
// crate名与package名相同
fn main() {
    println!("Hello, world!");
}
