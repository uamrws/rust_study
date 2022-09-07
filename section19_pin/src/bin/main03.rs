#![allow(unused)]
use std::{thread, time::Duration};

use futures::{executor::block_on, join, try_join};
struct Book;
struct Music;

async fn get_book() -> Result<Book, String> {
    /* ... */
    println!("book");
    Err("Book".to_string())
}
async fn get_music() -> Result<Music, String> {
    /* ... */
    thread::sleep(Duration::from_secs(1));
    println!("music");
    Ok(Music)
}

async fn get_book_and_music() {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut);
}

fn main() {
    block_on(get_book_and_music());
}
