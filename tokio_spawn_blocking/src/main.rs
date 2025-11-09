use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    tokio::join!(delay(1, 1), delay(2, 2), delay(3, 3),);

    println!("Main finished");
}

async fn delay(task: u64, time: u64) {
    println!("Task {task} starts");

    let result = spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_secs(time));
        println!("Task {task} blocking...");
        time
    })
    .await;
    println!("Result is {result:#?}");

    println!("Task {task} ends");
}
