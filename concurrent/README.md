## 进程与线程
- 在大部分OS里，代码运行在进程(process)中，OS同时管理多个进程
- 在你的程序里，各独立部分可以同时运行，运行这些独立部分的就是线程(thread)
- 多线程运行:
  - 提升性能表现
  - 增加复杂性，无法保障各线程的执行顺序

## 多线程可导致的问题
- 竞争状态，线程以不一致的顺序访问数据或资源
- 死锁，两个线程彼此等待对方使用完所持有的资源，线程无法继续
- 只在某些情况下发生的bug, 很难可靠地复制现象与修复

## 实现线程的方式
- 通过调用OS的API来创建线程: 1:1模型
  - 需要较小的运行时
- 语言自己实现的线程(绿色线程)：M:N模型
  - 需要更大的运行时

- Rust: 需要权衡运行时的支持
- Rust标准库仅支持1：1模型

## 使用spawn创建新线程
```rust
use std::thread;
use std::time::Duration;

fn main() {
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}
```

## 使用join等待所有线程结束
```rust
use std::thread;
use std::time::Duration;

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // 会阻塞主线程的执行，直到thread::spawn出的子线程执行完成
  handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}
```


## 消息传递

- 一种很流行且能保证安全和并发的技术就是：消息传递
  - 线程（或 Actor）通过彼此发送消息（数据）来进行通信
- GO 语言的名言：不要用共享内存来通行，要用通信来共享内存
- Rust： Channel（标准库提供）

## 创建 Channel

- 使用 mpsc::channel 函数来创建 Channel
  - mpsc 表示 multiple producer, single consumer(多个生产者，一个消费者)
  - 返回一个 tuple(元组): 里面元素分别是发送端，接收端

## 发送端的 send 方法

- 参数: 想要发送的数据
- 返回: Result<T,E>
  - 如果有问题(例如接收端已经被丢弃)，就返回一个错误

## 接收端的方法

- recv 方法: 阻止当前线程执行，直到 Channel 中有值被送来
  - 一旦有值收到，就返回 Result<T, E>
  - 当发送端关闭，就会收到一个错误
- try_recv 方法: 不会阻塞
  - 立即返回 Result<T, E>:
    - 有数据到达，返回 ok, 里面包含着数据
    - 否则，返回错误
  - 通常会使用循环调用来检查 try_recv 的结果

## Channel 和所有权转移

- 所有权在消息传递中非常重要，能帮你编写安全，并发的代码

## 通过 mpsc::Sender::clone 可以创建多个发送者

## 使用共享来实现并发
- GO语言的名言: 不要用共享内存来通信，要用通信来共享内存
- Rust支持通过共享状态来实现并发
- Channel类似单所有权：一旦将值得所有权转移至channel, 就无法使用它了
- 共享内存并发类似多所有权: 多个线程可以同时访问同一块内存

## 使用 Mutex 来每次只允许一个线程来访问数据

- Mutex 是 mutual exclusion（互斥锁）的缩写
- 在同一时刻，Mutex 只允许一个线程来访问某些数据
- 想要访问数据:
  - 线程必须首先获取互斥锁(lock)
    - lock 数据结构是 mutex 的一部分，它能跟踪谁对数据拥有独占访问权

## Mutex 的两条规则

- 在使用数据之前，必须尝试获取锁(lock)
- 使用完 mutex 所保护的数据，必须对数据进行解锁，以便其它线程可以获取锁

## Mutex<T>的 API

- 通过 Mutex::new（数据）来创建 Mutex<T>
  - Mutex<T>是一个智能指针
- 访问数据前，通过 lock 方法来获取锁
  - 会阻塞当前线程
  - lock 可能会失败
  - 返回的是 MutexGuard（智能指针，实现了 Deref 和 Drop）

  ## 使用Arc<T>来进行原子引用计数
  - Arc<T>与Rc<T>类似，它可以用于并发情景
    - A:atomic,原子的
  - 为什么所有的基础类型都不是原子的，为什么标准库类型不默认使用Arc<T>?
    - 需要性能作为代价
  - Arc<T>和Rc<T>的API是相同的

## RefCell<T>/Rc<T> VS Mutex<T>/Arc<T>
- Mutex<T>提供了内部可变性，和Cell家族一样
- 我们使用RefCell<T>来改变Rc<T>里面的内容
- 我们使用Mutex<T>来改变Arc<T>里面的内容
- 注意:Mutex<T>有死锁风险