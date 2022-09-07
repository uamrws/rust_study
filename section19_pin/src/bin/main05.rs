#![allow(unused)]

use futures::executor::block_on;
fn main() {
    use futures::{
        future::{Fuse, FusedFuture, FutureExt},
        pin_mut, select,
        stream::{FusedStream, Stream, StreamExt},
    };

    async fn get_new_num() -> u8 {
        /* ... */
        5
    }

    async fn run_on_new_num(_: u8) { /* ... */
    }

    async fn run_loop(
        mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
        starting_num: u8,
    ) {
        let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
        let get_new_num_fut = Fuse::terminated();
        pin_mut!(run_on_new_num_fut, get_new_num_fut);
        loop {
            select! {
                () = interval_timer.select_next_some() => {
                    // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                    if get_new_num_fut.is_terminated() {
                        get_new_num_fut.set(get_new_num().fuse());
                    }
                },
                new_num = get_new_num_fut => {
                    // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                    run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
                },
                // 运行 `run_on_new_num_fut`
                () = run_on_new_num_fut => {},
                // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
                //后，执行到 `complete` 分支
                complete => panic!("`interval_timer` completed unexpectedly"),
            }
        }
    }
    
}
