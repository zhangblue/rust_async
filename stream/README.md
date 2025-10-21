# Stream

“消息传递”中异步的`recv`方法会随着时间产生一系列项目，这称为`stream`
| | Async Channel receiver<br />异步通道接收器 | Iterator<br />迭代器 |
| :----------: | :----------------------------------------: | :------------------: |
| 同步 or 异步 | 异步的 | 同步的 |
| API | `recv()`, (`trpl::Receiver`)              | `next()`             |

`Stream` 就像异步版本的`Iterator`

可以从任何`iterator`来创建`Stream`

## StreamExt

Ext是Rust社区中使用另外一个`trait`扩展某个trait的常见模式

- 简单来说
    - `Stream trait`定义了一个低级接口，有效的结合了`iterator`和`Future` traits
    - `StreamExt`在`Stream`之上提供了一组更高级的API，包括next方法以及类似于`Iterator` trait提供的其他实用方法
- `Stream`和`StreamExt`尚未称为Rust标准库的一部分，但大多数生态系统中的crate使用相同的定义

## Composing Stream

组合流

- 许多概念天然适合`stream`来表示：
    - 队列中逐渐可用的项目
    - 文件系统中逐步拉取的数据块(数据集太大时)
    - 网络上随时间到达的数据
    - 实时通信(如WebSocket)
- `Streams`其实就是`Futures`，可以与任意类型的`Future`组合使用
