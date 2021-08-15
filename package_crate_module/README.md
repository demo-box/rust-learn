## crate 的类型

- binary crate
- library crate

## crate root

- 是源代码文件
- rust 编译器从这里开始，组成你的 crate 的根 module

## 一个 package

- 包含 1 个 cargo.toml, 描述如何构建这些 crates
- 只能包含 0-1 个 library crate
- 可以包含任意数量的 binary crate
- 但必须至少包含一个 crate (library 或 binary)

## module

- 在一个 crate 内，将代码进行分组
- 增加可读性，易于复用
- 控制项目（item）的私有性。public, private

## 路径(Path)

- 为了在 Rust 的模块中找到某个条目, 需要使用路径
- 路径的两种形式:
  - 绝对路径: 从 crate root 开始，使用 crate 名或字面值 crate
  - 相对路径: 从当前模块开始，使用 self,super 或当前模块的标识符
- 路径至少由一个标识符组成，标识符之间使用::

## pub struct

- pub 放在 struct 前:
  - struct 是公共的
  - struct 的字段默认是私有的

## pub enum

- pub 放在 enum 前:
  - enum 是公共的
  - enum 的变体也都是公共的
