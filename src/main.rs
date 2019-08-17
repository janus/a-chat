#![feature(async_await)]

use std::net::ToSocketAddrs;

use async_std::{
    prelude::*,
    task,
    net::TcpListener,
    net::TcpStream,
    io::{stdin, BufReader}
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let _handle = spawn_and_log_error(task::spawn(client(stream)));

    }
    Ok(())
    // add code here
}

async fn client(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name = {}", name);

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
        };
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
        let msg: String = msg.trim().to_string();
    }
    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{} Joosk", e)
        }
    })
}

fn main()-> Result<()> {
    let fut = server("127.0.0.1::8080");
    task::block_on(fut)
}
