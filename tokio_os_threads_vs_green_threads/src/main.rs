#[tokio::main]
async fn main() {
    tokio::spawn(run());
    hello().await;

    join_demo::demo().await;
    join_set_demo::demo().await;
}

async fn hello() {
    println!("Hello, world!");
}

async fn run() {
    for i in 0..10 {
        println!("{i}");
    }
}

async fn add(a: i32, b: i32) -> i32 {
    println!("{}", a + b);
    a + b
}

mod join_set_demo {
    use crate::add;
    use tokio::task::JoinSet;

    pub async fn demo() {
        let mut set = JoinSet::new();

        // 将future放入set中
        for i in 0..10 {
            set.spawn(add(i, 2));
        }

        // 循环得到set中的结果
        while let Some(result) = set.join_next().await {
            println!("{:?}", result);
        }
    }
}

mod join_demo {
    use crate::add;

    pub async fn demo() {
        let result = tokio::join!(add(1, 2), add(2, 3), add(3, 4));
        println!("{:?}", result);

        let _ = tokio::join!(
            tokio::task::spawn(add(1, 2)),
            tokio::task::spawn(add(2, 3)),
        );
    }
}
