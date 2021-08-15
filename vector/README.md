## 使用 Vector 存储多个值

- Vec<T>,叫做 vector
  - 由标准库提供
  - 可存储多个值
  - 只能存储相同类型的数据
  - 值在内存中连续存放

## 创建 Vector

- Vec::new 函数

- 使用初始值创建 Vec<T>, 使用 vec!宏

## 更新 Vector

- 向 Vector 添加元素，使用 push 方法

## 删除 Vector

- 与任何其它 struct 一样，当 Vector 离开作用域后
  - 它就被清理掉了
  - 它所有的元素也被清理掉了

## 读取 Vector 的元素

- 两种方式可以引用 Vector 里的值
  - 索引: 超出 vector 边界的时候，会产生 panic
  - get 方法: 超过 vector 边界的时候，不会产生 panic

## 使用 enum 来存储多种数据类型

- Enum 的变体可以附加不同类型的数据
- Enum 的变体定义在同一个 enum 类型下
