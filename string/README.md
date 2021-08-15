## 字符串是什么

- Byte 的集合

  - 有一些方法
  - 能将 byte 解析为文本

- Rust 的核心语言层面，只有一个字符串类型: 字符串切片 str (或&str)
- 字符串切片: 对存储在其它地方，utf-8 编码的字符串的引用

  - 字符串字面值: 存储在二进制文件中，也是字符串切片

- String 类型
  - 来自标准库而不是核心语言
  - 可增长，可修改，可拥有所有权
  - utf-8 编码

## 通常说的字符串是指?

- String 和&str
  - 标准库里用的多
  - utf-8 编码

## 其它类型的字符串

- rust 的标准库还包含了很多其它的字符串类型，例如: OsString, OsStr, CString, CStr

## 创建一个新的字符串(String)

- 很多 Vec<T>的操作都可用于 String
- String::new()函数
- 使用初始值来创建 String:
  - to_string()方法，可用于实现了 Display trait 的类型，包含字符串字面值
  - String::from()函数，从字面值创建 String

## 更新 String

- push_str()方法: 把一个字符串切片附加到 String
- push 方法: 把单个字符附加到 String

## + 号连接字符串

- 使用了类似这个签名的方法 fn add(self, s: &str) -> String {}
  - 标准库中的 add 方法使用了泛型
  - 只能把&str 添加到 String
  - 解引用强制转换(deref coercion)

## 字节(Bytes)，标量值(Scalar Values)，字形簇(Grapheme Clusters)

- Rust 有三种看待字符串的方式

  - 字节
  - 标量值
  - 字形簇(最接近所谓的"字母")

- Rust 不允许对 String 进行索引
  - 索引操作应消耗一个常量时间(O(1))
  - 而 String 无法保证：需要遍历所有内容，来确定有多少个合法的字符
