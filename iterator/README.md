## iterator trait

- 所有迭代器都实现了 iterator trait
- iterator trait 定义于标准库，定义大致如下:

```javascript
pub trait iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
```

- type Item 和 Self::Item 定义了与此 trait 关联的类型

- iterator trait 仅要求实现一个方法: next

- next:

  - 每次返回迭代器中的一项
  - 返回结果包裹在 Some 里
  - 迭代结束，返回 None

- 可直接在迭代器上调用 next 方法

## 几个迭代方法

- iter 方法: 在不可变引用上创建迭代器
- into_iter 方法: 创建的迭代器会获得所有权
- iter_mut 方法: 迭代可变的引用

## 消耗迭代器的方法

- 在标准器中，iterator trait 有一些带默认实现的方法
- 其中有一些方法会调用 next 方法
  - 实现 iterator trait 时必须实现 next 方法的原因之一
- 调用 next 的方法叫做'消耗性适配器'
  - 因为调用它们会把迭代器消耗尽
- 例如： sum 方法(就会耗尽迭代器)
  - 取得迭代器的所有权
  - 通过反复调用 next，遍历所有元素
  - 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和

## 产生其它迭代器的方法

- 定义在 iteractor trait 上的另一些方法叫做'迭代器适配器'

  - 把迭代器转换为不同种类的迭代器

- 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性较高

- 例如: map

  - 接收一个闭包，闭包作用于每次元素
  - 产生一个新的迭代器

- collect 方法: 消耗性适配器，把结果收集到一个集合类型中

## 零开销抽象 Zero-Cost Abstraction

- 使用抽象时不会引入额外的运行时开销
