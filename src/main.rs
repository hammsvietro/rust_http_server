use tokio::net::TcpListener;
use async_std::task::block_on;
use tokio::runtime::Runtime;

mod http;
mod event_loop;
use event_loop::execute_loop;


fn main() {
    let rt = Runtime::new().unwrap();
    block_on(rt.spawn(async {
        let listener =TcpListener::bind("127.0.0.1:8080").await.unwrap();
        execute_loop(&listener).await
    }));
    
}