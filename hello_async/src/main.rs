use trpl::{Either, Html};

/// 需求：一个命令行工具，提取两个网页的title，哪个网页先提取完就先打印出来
/// cargo run -- https://www.rust-lang.org https://www.google.com/
fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let title_future_url1 = page_title(&args[1]);
        let title_future_url2 = page_title(&args[2]);

        // trpl::race() 表示两个future竞争，谁先完成就返回谁的内容。慢的那个会被取消
        let (url, maybe_title) = match trpl::race(title_future_url1, title_future_url2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            None => println!("{url} has no title"),
            Some(title) => println!("The title for {url} was {title}"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await; // 返回响应的的文本内容
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
