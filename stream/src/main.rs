fn main() {
    // sample_stream::steam_demo1();
    // sample_stream::steam_demo2();
    // composing_stream::composing_stream_demo1();
    composing_stream::merge_stream_demo();
}

mod sample_stream {
    use trpl::StreamExt;

    pub fn steam_demo1() {
        trpl::run(async {
            let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let iter = values.iter().map(|n| n * 2);
            let mut stream = trpl::stream_from_iter(iter);
            while let Some(value) = stream.next().await {
                println!("The value was {value}");
            }
        })
    }

    pub fn steam_demo2() {
        trpl::run(async {
            let values = 1..101;
            let iter = values.map(|n| n * 2);
            let stream = trpl::stream_from_iter(iter);

            let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

            while let Some(value) = filtered.next().await {
                println!("The value was {value}");
            }
        })
    }
}

mod composing_stream {
    use std::pin::pin;
    use std::time::Duration;
    use trpl::{ReceiverStream, Stream, StreamExt};

    /// 给获取流添加超时时间
    pub fn composing_stream_demo1() {
        trpl::run(async {
            let mut messages = pin!(
                get_message().timeout(Duration::from_millis(200)) // 对流获取的数据添加延迟
            );
            while let Some(result) = messages.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                }
            }
        })
    }

    fn get_message() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel::<String>();
        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];
            for (index, message) in messages.into_iter().enumerate() {
                // 给发送数据添加延迟
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;
                if let Err(send_err) = tx.send(format!("Message: {message}")) {
                    eprintln!("Cannot send message '{message}': {send_err}");
                    break;
                }
            }
        });
        ReceiverStream::new(rx)
    }

    /// 将两个流进行合并
    pub fn merge_stream_demo() {
        trpl::run(async {
            let message = get_message().timeout(Duration::from_millis(200));
            let intervals = get_intervals()
                .map(|count| format!("Interval: {count}"))
                .throttle(Duration::from_millis(100)) // 截流，限制这个流被调用的频率没100毫秒调用一次。如果这里不限制，message这个流的数据会被淹没在intervals流的数据中
                .timeout(Duration::from_secs(10));

            let merge = message.merge(intervals).take(20); // 限制从流中最多拿20条数据
            let mut stream = pin!(merge);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                }
            }
        })
    }

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel::<u32>();
        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                if let Err(send_err) = tx.send(count) {
                    eprintln!("Cannot send interval {count}: {send_err}");
                    break;
                }
            }
        });

        ReceiverStream::new(rx)
    }
}
