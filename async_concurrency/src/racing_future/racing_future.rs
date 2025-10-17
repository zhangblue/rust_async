use std::time::Duration;

/// race()函数使两个future相互竞争，哪个future先执行完算谁的，不会等待慢的future的执行
pub fn racing_future_demo() {
    trpl::run(async {
        let slow = async {
            println!("slow started");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("slow finished");
        };

        let fast = async {
            println!("fast started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("fast finished");
        };

        trpl::race(slow, fast).await;
    });
}
