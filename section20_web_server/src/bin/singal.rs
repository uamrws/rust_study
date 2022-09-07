use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listerner = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    loop {
        let (stream, addr) = listerner.accept().await.unwrap();
        tokio::spawn(async move { handle_connection(stream, addr).await });
    }
}

async fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {

    println!("client connection:{addr}");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "section20_web_server/hello.html")
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "section20_web_server/404.html",
        )
    };
    let content = tokio::fs::read_to_string(filename).await.unwrap();
    let response = format!("{status_line}{content}");
    // println!("client response:{response}");
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
