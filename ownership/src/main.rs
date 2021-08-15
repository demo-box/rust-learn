fn main() {
    {
        let s1 = String::from("hello");
        println!("s1 = {}", s1);

        // move
        let s2 = s1; // move是为了解决内存二次释放问题
                     // println!("s1 = {}", s1);
        println!("s2 = {}", s2);

        // clone, 深拷贝了堆上的内存
        let s3 = s2.clone();
        println!("s2 = {}", s2);
        println!("s3 = {}", s3);

        // copy trait
        let a = 1;
        let b = a;
        println!("a = {}, b = {}", a, b);
        // 常用的具有copy trait的类型: 1. 整型 2. 浮点型 3.布尔型 4. 字符型 5. 元组（如果其所有字段都是可以copy的）
    }

    // 借用
    // 我们把引用作为函数参数这个行为叫做借用
}
