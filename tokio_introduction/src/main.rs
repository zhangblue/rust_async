fn main() {
    current_thread::current_thread_rt().block_on(hi());
    multi_thread::multi_thread_rt().block_on(hi());
}

async fn hi() {
    println!("Hello tokio!");
}

mod current_thread {

    pub fn current_thread_rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    }
}

mod multi_thread {
    pub fn multi_thread_rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(10) // 线程数量
            .thread_stack_size(5 * 1024 * 1024) // 线程栈大小
            .max_blocking_threads(256) //阻塞线程的数量
            .enable_all()
            .build()
            .unwrap()
    }
}
