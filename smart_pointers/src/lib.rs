use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::{CustomSmartPointer, MyBox};

    #[test]
    fn one() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // 用*解引用
    }

    #[test]
    fn deref_coercion() {
        let m = MyBox::new(String::from("Rust"));

        // &m通过deref coercion进行来一系列自动转换
        // deref &m -> &String
        // deref &String -> &str
        hello(&m);
        hello("rust");
    }

    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    #[test]
    fn drop_trait() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };

        drop(c);
        println!("CustomSmartPointer created.");
    }
}
