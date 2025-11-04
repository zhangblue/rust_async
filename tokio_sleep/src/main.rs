use std::thread;
use std::time::Duration;

async fn hello(task: u64, time: u64) {
    println!("task:{},started on {:?}", task, std::thread::current().id());
    // thread::sleep(Duration::from_millis(time)); // 标准库的sleep并不会让出CPU控制权，导致后卡住后续逻辑
    tokio::time::sleep(Duration::from_millis(time)).await; // 使用tokio的异步sleep，此时会让出CPU控制权，此时CPU就可以去执行其他的异步任务了，从而达到并发目的
    println!("task:{}, has finished", task);
}

#[tokio::main]
async fn main() {
    tokio::join!(hello(1, 200), hello(2, 200), hello(3, 200), hello(4, 200),);
}
