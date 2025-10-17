use std::time::Duration;
use trpl::Either;

/// 需求：构建一个timeout函数，用于实现某个future最长运行多久，如果这个future没有超过设置的时间，则返回这个future执行的结果，否则返回Error信息

pub fn timeout_demo() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(200)).await {
            Ok(message) => println!("Success with {message}"),
            Err(duration) => {
                println!("Failed after {} millis", duration.as_millis());
            }
        }
    })
}

/// 函数作用：如果这个future可以在规定的timeout时间内完成，那就返回这个future的执行结果，否则返回一个错误信息。
async fn timeout<F>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration>
where
    F: Future,
{
    // race()函数，是对两个future进行竞争，谁先完成就返回谁
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use trpl::sleep;

    /// 有个sleep函数，可以睡眠指定的时间，那么这段代码将大约执行多少秒.
    /// 答案：15秒
    #[test]
    fn homework_1() {
        trpl::run(async {
            println!("begin");
            let start = Instant::now();
            let futs: Vec<_> = [1, 2, 3]
                .iter()
                .map(|n| async move {
                    println!("执行第{}个", n);
                    sleep(Duration::from_secs(5)).await;
                    n + 1
                })
                .collect();
            let time = start.elapsed();
            println!("futs time: {:?}", time);

            println!("begin1");
            let now = Instant::now();
            for fut in futs {
                let n = fut.await;
                println!("{:?}", n);
            }
            let time = now.elapsed();
            println!("use time: {:?}", time.as_secs());
        })
    }
}
