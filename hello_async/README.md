# Futures和异步语法

## 核心元素

- `Future`: 一个可能现在还未准备好(就绪),但将来会准备好(就绪)的值
    - 在其他语言中也称为 task 或 promise
    - 在Rust中，Future是实现了 Future trait 的类型
- `async`关键字：用于代码块或函数，来表示可能被中断和恢复
    - 将函数或代码块转换为返回`Future`的形式
- `await`关键字：用于等待`Future`就绪
    - 提供暂停和恢复执行的点
    - "轮询"(polling)是检查`Future`值是否可用的过程

## Future的特点

- Rust 编译器将`async/await`代码转换为使用`Future trait`的等效代码
    - 类似于`for`循环被转换为使用`Iterator trait`
- 开发者可以为自定义数据类型实现`Future trait`
    - 提供统一接口但允许不同的异步操作实现

`trpl` 这个crate，整合了我们需要的类型、`trait`和函数，主要来自`future`和`tokio`这两个核心异步库

目标：专注于异步编程学习，避免生态系统干扰

工具：使用`trpl`库(The Rust Programming Language)

- 整合`future`和`tokio`的核心功能
- `future`:异步实验的官方家园，定义了`Future`特性
- `tokio`:最流行的异步运行时，广泛应用于Web开发

设计：

- `trip`重导出类型、函数和`trait`，简化学习
- 隐藏复杂细节，专注于异步核心