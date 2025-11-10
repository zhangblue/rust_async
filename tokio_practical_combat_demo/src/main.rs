use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // 设置日志级别为info
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // 设置全局默认的订阅者，这样我们就可以在程序的任何地方使用日志了
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("服务器启动, 监听8080端口...");
    let (tx, _) = broadcast::channel(10);

    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let (mut socket, addr) = listener.accept().await.unwrap();
        tracing::info!("客户端连接：{}", addr);
        tokio::spawn(async move {
            // 将socket拆分为读写两部分，以便分别处理读取和写入操作
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            // 读取客户端发送的数据到缓冲区中
            let mut reader = BufReader::new(stream_reader);
            loop {
                // select! 宏用于同时等待多个异步操作完成，并执行第一个完成的操作的代码块。
                // select！模式匹配：pattern = future => handler
                tokio::select! {
                    // 读取用户在终端上发送的数据到缓冲区中
                    result = reader.read_line(&mut message) => {
                        tracing::info!("接收到客户端发送的消息:{}",&message);
                        // 如果读取到0个字节，则断开连接，说明终端已经断开连接
                        if result.unwrap()==0{
                            break;
                        }
                        // 将输入的内容通过广播发送给其他所有客户端
                        tx.send((message.clone(),addr)).unwrap();
                        message.clear();
                    }
                    // 接收到其他客户端广播来的消息，并将消息显示在当前终端上
                    result = rx.recv() => {
                        let (received_message,sender_address) = result.unwrap();
                        // 如果不是自己发送的消息，则显示出来
                        if sender_address!=addr{
                            tracing::info!("接收到其他客户端发送的消息:{}",&received_message);
                            stream_writer.write_all(received_message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
