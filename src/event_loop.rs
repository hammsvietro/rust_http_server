
use crate::http::HttpRequest;

use std::io::Error;
use async_std::fs;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};


pub async fn execute_loop(listener: &TcpListener)-> Result<(), Error> {
  loop {
    let (mut socket, _) = listener.accept().await?;

    tokio::spawn(async move {
      let mut buf = [0; 1024];

      loop {
        let n = match socket.read(&mut buf).await {
          Ok(n) if n == 0 => return,
          Ok(n) => n,
          Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return;
          }
        };

        if let Some(request) = std::str::from_utf8(&buf).unwrap().split("\n").into_iter().next() {
          println!("request:");
          println!("{}",request);
        }
        let file = fs::read_to_string("www/hello.html").await.unwrap();
        if let Err(e) = socket.write_all(format!("HTTP/1.1 200 OK\r\n\r\n{}",file).as_bytes()).await {
          eprintln!("failed to write to socket; err = {:?}", e);
          return;
        }
      }
    });
  }
}


fn parse_request<'a>(req: &'a str)  {

}