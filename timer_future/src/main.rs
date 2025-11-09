use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::mpsc::{Receiver, SyncSender, sync_channel},
        sync::{Arc, Mutex},
        task::Context,
        time::Duration,
    },
    // 引入之前实现的定时器模块
    timer_future::TimerFuture,
};

/// 梳理了下整个executor + TimerFuture的流程：
/// 1.executor run从队列中获得的是async块生成的future内容，poll执行async同步代码，输出howdy
/// 2.executor run poll内继续执行，遇到TimerFuture.await，执行TimerFuture poll，返回pending状态
/// 3.TimerFuture执行完毕，awake唤醒executor run，再次执行executor run poll，然后进入TimerFuture poll
/// 4.TimerFuture poll直接返回ready状态，executor run poll得以继续async内await下面的代码，输出done
/// 5.输出done后，此时executor run poll才算执行完毕，返回ready状态，完成
///
/// 总结一下几个要点：
///
/// async作为一个future，每次poll遇到await停止，进入await方法里的poll
/// await方法内awake唤醒的是上层调度逻辑，而不是await方法本身
/// 唤醒调度逻辑后会继续在上层调度逻辑进行poll，然后再次进入await方法里的poll直到该await方法ready
/// 当检测到await方法ready，那么该await方法就相当于poll内的同步代码了，直接往下走
///
/// 再次总结一下async内碰到await的逻辑：
///
/// async碰到await，会沉入到await方法内的poll
/// await poll pending，那么记录当前future，等待唤醒
/// await poll ready，则await执行完毕，继续同步代码直到再次await
///

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy");
        // 创建定时器Future，等待他完成. 此处只有调用await，才会触发poll函数推进TimerFuture的进度。
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);
    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// 进行中的Future，在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("通过wake唤醒任务，将自己重新放回队列中，放入后可以让Executor重新获取到这个任务");
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 从队列中获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                let poll_res = future.as_mut().poll(context);
                if poll_res.is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}
