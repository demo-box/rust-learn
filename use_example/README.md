## use 的习惯用法

- 函数: 将函数的父级模块引入作用域(指定到父级)
- struct, enum, 其它: 指定完整路径(指定到本身)
- 同名条目: 指定到父级
- 通过 as 关键字可以设置别名

## 将模块内容移动到其它文件

- 模块定义时，如果模块名后边是";", 而不是代码块:

  - Rust 会从模块同名的文件中加载内容
  - 模块树的结构不会变化

- 随着模块逐渐变大，该技术让你可以把模块的内容移动到其它文件中
