use async_std:: {
    prelude::*,
    task,
    net::{TcpListener, ToSocketAddrs},
}


async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        // TODO
    }
    Ok(())
}

fn run() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync >>;
