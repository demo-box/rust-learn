## 消息传递
- 一种很流行且能保证安全和并发的技术就是：消息传递
    - 线程（或Actor）通过彼此发送消息（数据）来进行通信
- GO语言的名言：不要用共享内存来通行，要用通信来共享内存
- Rust： Channel（标准库提供）

## 创建Channel
- 使用mpsc::channel函数来创建Channel
    - mpsc表示multiple producer, single consumer(多个生产者，一个消费者)
    - 返回一个tuple(元组): 里面元素分别是发送端，接收端

## 发送端的send方法
- 参数: 想要发送的数据
- 返回: Result<T,E>
    - 如果有问题(例如接收端已经被丢弃)，就返回一个错误

## 接收端的方法
- recv方法: 阻止当前线程执行，直到Channel中有值被送来
    - 一旦有值收到，就返回Result<T, E>
    - 当发送端关闭，就会收到一个错误
- try_recv方法: 不会阻塞
    - 立即返回Result<T, E>:
        - 有数据到达，返回ok, 里面包含着数据
        - 否则，返回错误
    - 通常会使用循环调用来检查try_recv的结果

## Channel和所有权转移
- 所有权在消息传递中非常重要，能帮你编写安全，并发的代码

## 通过mpsc::Sender::clone可以创建多个发送者