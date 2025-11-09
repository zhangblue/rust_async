# Tokio Spawn_blocking(...)

- 如果想再 Tokio 里执行阻塞操作
    - 到没有 async 接口设备的I/O操作
    - CPU密集型任务
    - ... 其他无法async的任务
- Tokio 提供 `spawn_blocking` 函数