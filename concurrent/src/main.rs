use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // let (tx2, rx2) = mpsc::channel();
    // 通过clone创建另一个发送者
    let tx2 = mpsc::Sender::clone(&tx);
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // rx可以作为一个迭代器使用，就无需手动调用rx.recv()了
    for received in rx {
        println!("Got: {}", received);
    }
}
