#![allow(unused)]
fn main() {
    async fn get_two_sites_async() {
        // 创建两个不同的`future`，你可以把`future`理解为未来某个时刻会被执行的计划任务
        // 当两个`future`被同时执行后，它们将并发的去下载目标页面
        let future_one = download_async("https://www.foo.com");
        let future_two = download_async("https://www.bar.com");

        // 同时运行两个`future`，直至完成
        join!(future_one, future_two);
    }
}
