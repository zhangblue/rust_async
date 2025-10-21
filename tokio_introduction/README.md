# Tokio 简介

Tokio 是Rust编程语言的一个异步运行时

- 提供了编写网络应用所需的构建块。它具有灵活性，可以针对各种系统，从拥有数十个内核的大型服务器到小型嵌入式设备
- 几个主要组件：
    - 一个用于执行异步代码的多线程运行时
    - 标准库的异步版本
    - 一个庞大的库生态系统

## 单线程模式

使用宏：`#[tokio::main(flavor="current_thread")]`

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello world");
}
```

可以使用`cargo expand`命令对使用宏生成的编译后的代码进行查看，发现与远程代码几乎相同

或者使用原生代码：`block_on(...)`

```rust
    pub fn current_thread_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
```

## 多线程模式

- 默认情况下，`Tokio`会为每个CPU核心启动一个线程(可自行控制)

- 每个线程都有自己独立的任务队列(`task list`)

- 每个线程都有自己的反应器(`reactor`)，即事件循环。

- 每个线程都支持工作窃取(`work stealing`)

- 你也可以配置线程数量，以及每个线程的事件循环数量

  ```rust
      pub fn multi_thread_rt() -> tokio::runtime::Runtime {
          tokio::runtime::Builder::new_multi_thread()
              .worker_threads(10) // 线程数量
              .thread_stack_size(5 * 1024 * 1024) // 线程栈大小
              .max_blocking_threads(256) //阻塞线程的数量
              .enable_all()
              .build()
              .unwrap()
      }
  ```

## 并发(Concurrency) vs 并行(Parallelism)

- 并发：在一个CPU/一个CPU内核/一个线程上，处理器在多个任务之间交替执行。
- 并行：当有多个处理器时，多个任务可以在同一时刻在不同核心上一起执行



