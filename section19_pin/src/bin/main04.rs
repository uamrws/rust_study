#![allow(unused)]

use futures::executor::block_on;
fn main() {
    use futures::{
        future::FutureExt, // for `.fuse()`
        pin_mut,
        select,
    };

    async fn task_one() { /* ... */
    }
    async fn task_two() { /* ... */
    }

    async fn race_tasks() {
        let t1 = task_one().fuse();
        let t2 = task_two().fuse();

        pin_mut!(t1, t2);

        select! {
            () = t1 => println!("任务1率先完成"),
            () = t2 => println!("任务2率先完成"),
        }
    }
    block_on(race_tasks());
}
