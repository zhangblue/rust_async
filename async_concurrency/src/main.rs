mod racing_future;
mod timeout_demo;

use crate::racing_future::racing_future_demo;
use crate::timeout_demo::timeout_demo;
use std::pin::Pin;
use std::time::Duration;

fn main() {
    // demo1();
    // demo2();
    racing_future_demo();
    timeout_demo();
}

fn demo1() {
    trpl::run(async {
        let future1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let future2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(future1, future2).await;
    })
}

/// 需求：使用消息传递在两个异步函数中进行计数
fn demo2() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel::<String>();

        let tx1 = tx.clone();

        let tx1_future = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_future = async {
            while let Some(received) = rx.recv().await {
                println!("收到 {received}");
            }
        };

        let tx2_future = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = vec![
            Box::pin(tx1_future),
            Box::pin(rx_future),
            Box::pin(tx2_future),
        ];

        trpl::join_all(futures).await;
    })
}
