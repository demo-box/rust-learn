## 什么是闭包(closures)

- 闭包: 可以捕获其所在环境的匿名函数
- 闭包:
  - 是匿名函数
  - 保存为变量，作为参数
  - 可在一个地方创建闭包，然后在另一个上下文中调用闭包完成运算
  - 可从其定义的作用域捕获值

## 闭包的类型推断

- 闭包不要求标注参数和返回值的类型
- 闭包通常很短小，只在狭小的上下文中工作，编译器通常能推断出类型
- 可以手动添加类型标注

## 闭包的类型推断

- 注意: 闭包的定义最终只会为参数/返回值推断出唯一具体的类型

## 如何让 struct 持有闭包

- struct 的定义需要知道所有字段的类型
  - 需要指明闭包的类型
- 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名完全一样
- 所以需要使用: 泛型和 Trait Bound

## Fn Trait

- Fn traits 由标准库提供
- 所有的闭包都至少实现了以下 trait 之一:
  - Fn
  - FnMut
  - FnOnce

## 闭包可以捕获他们所在的环境

- 闭包可以访问定义它的作用域内的变量，而普通函数则不能
- 闭包会产生内存开销

## 闭包从所在环境捕获值的方式

- 与函数获得参数的三种方式一样:

1. 取得所有权: FnOnce
2. 可变借用: FnMut
3. 不可变借用: Fn

- 创建闭包时，通过闭包对环境值的使用，Rust 推断出具体使用哪个 trait:
  - 所有的闭包都实现了 FnOnce
  - 没有移动捕获变量的实现了 FnMut
  - 无需可变访问捕获变量的闭包实现了 Fn

## move 关键字

- 在参数列表前使用 move 关键字，可以强制闭包取得它所使用的环境值的所有权
  - 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最为有用
