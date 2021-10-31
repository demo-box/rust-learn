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
