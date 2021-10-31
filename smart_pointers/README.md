## 相关的概念

- 指针: 一个变量在内存中包含的是一个地址(指向其它数据)
- rust 中最常见的指针是‘引用’
- 引用:
  - 使用 &
  - 借用它指向的值
  - 没有其余开销
  - 最常见的指针类型

## 智能指针

- 智能指针是这样一些数据结构
  - 行为和指针相似
  - 有额外的元数据和功能

## 引用计数(reference counting)智能指针类型

- 通过记录所有者的数量，使一份数据被多个所有者同时持有
- 并在没有任何所有者时自动清理数据

## 引用和智能指针的其它不同

- 引用: 只借用数据
- 智能指针: 很多时候都拥有它所指向的数据

## 智能指针的例子

- String 和 Vec<T>

- 都拥有一片内存区域，且允许用户对其操作

- 还拥有元数据(例如容量等)

- 提供额外的功能或保障(String 保障其数据是合法的 UTF-8 编码)

## 智能指针的实现

- 智能指针通常使用 struct 实现，并且实现了:

  - Deref 和 Drop 这两个 trait

- Deref trait: 允许智能指针 struct 的实例像引用一样使用
- Drop trait： 允许你自定义当智能指针实例走出作用域时的代码

## Box<T>

- Box<T>是最简单的智能指针

  - 允许你在 heap 上存储数据(而不是 stack)
  - stack 上是指向 heap 数据的指针
  - 没有性能开销
  - 没有其它额外功能

  - 实现了 Deref trait 和 Drop trait

## Box<T>的常用场景

- 在编译时，某类型的大小无法确定. 但使用该类型时，上下文却需要知道它的确切大小。
- 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制
- 使用某个值时，你只关心它是否实现了特定的 trait， 而不关心它的具体类型

## 使用 Box 赋能递归类型

- 在编译时，Rust 需要知道一个类型所占的空间大小
- 而递归类型的大小无法在编译时确定
- 但 Box 类型的大小确定
- 在递归类型中使用 Box 就可以解决上述问题

## 关于 Cons List

- Cons List 是来自 Lisp 语言的一种数据结构
- Cons List 里每个成员由两个元素组成

  - 当前项的值
  - 下一个元素

- Cons List 里最后一个成员只包含一个 Nil 值，没有下一个元素

![EZTXL____8F6_YFMY_25RR3.png](https://i.loli.net/2021/10/17/vnFLTwYEizalBVR.png)

## Cons List 并不是 Rust 的常用集合

- 通常情况下, Vec<T>是更好的选择

## 使用 Box 来获得确定大小的递归类型

- Box<T>是一个指针，Rust 知道它需要多少空间，因为:
  - 指针的大小不会基于它指向的数据的大小变化而变化

## 函数和方法的隐式解引用转化(Deref Coercion)

- 隐式解引用转化(Deref Coercion)是为函数和方法提供的一种便捷特性
- 假设 T 实现来 Deref trait：

  - Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用

- 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配:
  - Deref Coercion 就会自动发生
  - 编译器会对 deref 进行一系列调用，来把它转为所需的参数类型
    - 在编译时完成，没有额外性能开销

## 解引用与可变性

- 可使用 DerefMut trait 重载可变引用的\*运算符
- 在类型和 trait 在下列三种情况发生时，Rust 会执行 deref coercion：
  - 当 T: Deref<Target=U>，允许&T 转换为&U
  - 当 T：DerefMut<Target=U>, 允许&mut 转换为&mut U
  - 当 T: Deref<Target=U>, 允许&mut T 转换为&U

## Drop Trait

- 实现 Drop Trait，可以让我们自定义当值将要离开作用域时发生的动作

  - 例如: 文件，网络资源释放等
  - 任何类型都可以实现 Drop trait

- Drop trait 只要求你实现 drop 方法

  - 参数: 对 self 的可变引用

- Drop trait 在预导入模块里(prelude)

## 使用 std::mem::drop 来提前 drop 值

- 很难直接禁用自动的 drop 功能，也没必要
  - Drop trait 的目的就是进行自动的释放处理逻辑
- Rust 不允许手动调用 Drop trait 的 drop 方法
- 但可以调用标准库的 std::mem::drop 函数，来提前 drop 值
