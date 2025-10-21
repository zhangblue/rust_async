# 系统线程(OS Threads) vs 绿色线程(Green Threads)

|         系统线程<br />OS Threads          | 绿色线程<br />Green Threads |
|:-------------------------------------:|:-----------------------:|
|             更高效的利用多CPU/核心             |         轻量，开销小          |
| 开销更大：`Context Swtiching (线程切换)`，资源管理等 | 不依靠额外机制等活，难以高效利用多CPU/内核 |
|            创建大量OS线程会导致资源紧张            |   轻松创建成千上万，乃至百万级的并发任务   |
|              每个线程需要大量的内存              |        更具扩展性，高并发        |
|              阻塞操作，OS来处理               |      由运行时高效的处理阻塞操作      |
|           不同OS间的行为、性能可能差距很大           |      不同平台间一一致的并发模型      |

## Tokio常用的函数和宏

`spawn`、`join!`、`yield_now`

### join!

等待所有的`future`都运行完成后再返回

```rust
async fn add(a: i32, b: i32) -> i32 {
    println!("{}", a + b);
    a + b
}

let result = tokio::join!(add(1, 2), add(2, 3), add(3, 4));
println!("{:?}", result);
```

### join_set

- 将要执行的异步函数放入`JoinSet`中，使用`spawn()`放入异步函数后，就会出发函数执行，并且将执行结果也存入了`JoinSet`中-
- 调用`set.join_next()`来得到其中的执行结果

```rust
mod join_set_demo {
    use crate::add;
    use tokio::task::JoinSet;

    pub async fn demo() {
        let mut set = JoinSet::new();

        // 将future放入set中
        for i in 0..10 {
            set.spawn(add(i, 2)); // 放入spawn时就会触发异步函数执行
        }

        // 循环得到set中的结果
        while let Some(result) = set.join_next().await {
            println!("{:?}", result);
        }
    }
}
```

### yield_now()

`tokio::task::yield_now().await`

- 在执行CPU密集型任务时可以使用，可以强制程序交出CPU控制权。但最好的办法是将CPU密集型的任务放在一个单独的线程中运行，而不是使用异步
