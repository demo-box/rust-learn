## 定义一个 Trait

- Trait 的定义: 把方法签名放在一起，来定义实现某种目的所必需的一组行为
  - 关键字: trait
  - 只有方法签名,没有具体实现
  - trait 可以有多个方法；每个方法签名占一行，以；结尾
  - 实现该 trait 的类型必须提供具体的方法实现

## 在类型上实现 trait

- 与为类型实现方法类似
- 不同之处:
  - impl Xxxx for Tweet {}
  - 在 impl 的块里，需要对 Trait 里的方法签名进行具体实现 (不绝对，适用于大多数情况)

## 实现 trait 的约束

- 可以在某个类型上实现某个 trait 的前提条件是:
  - 这个类型或这个 trait 是在本地 crate 里定义的
- 无法为外部类型来实现外部的 trait
  - 这个限制是程序属性的一部分(也就是一致性)
  - 更具体地说是孤儿规则，之所以这样命名是因为父类型不存在
  - 此规则确保其他人的代码不能破坏您的代码，反之亦然
  - 如果没有这个规则，两个 crate 可以为同一类型实现同一个 trait, Rust 就不知道应该使用哪个实现了

## 默认实现

- 默认实现的方法可以调用 trait 中其它的方法，即使这些方法没有默认实现
- 注意: 无法从方法的重写实现里面调用该方法的默认实现

## Trait 作为参数类型约束

- impl Trait 语法: 适用于简单情况
- Trait bound 语法： 可用于复杂情况
  - impl Trait 语法是 Trait bound 的语法糖
- 使用 + 指定多个 Trait bound
- Trait bound 使用 where 子句

## Trait 作为返回类型约束

- impl Trait 语法
- 注意：impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错

## 使用 Trait bound 有条件的实现方法

- 在使用泛型类型参数的 impl 块上使用 Trait bound, 我们可以有条件的为实现了特定 Trait 的类型来实现方法

- 也可以为实现了其它 Trait 的任意类型有条件的实现某个 Trait
- 为满足 Trait bound 的所有类型上实现 Trait 叫做覆盖实现(blanket implementations)
