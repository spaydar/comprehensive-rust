#![allow(dead_code)]

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dining_philosophers().await
}

async fn take_call_wrapper() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("listening on port 6142");

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("connection from {addr:?}");

        async fn take_call(mut socket: tokio::net::TcpStream) -> Result<(),  std::io::Error> {
            socket.write_all(b"Who are you?\n").await?;
            
            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await?;
            let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
            socket.write_all(format!("Thanks for dialing in, {name}!\n").as_bytes()).await
        }

        let _ = take_call(socket).await;

        // // Async block implementation
        // tokio::spawn(async move {
        //     if let Err(e) = socket.write_all(b"Who are you?\n").await {
        //         println!("socket error: {e:?}");
        //         return;
        //     }

        //     let mut buf = vec![0; 1024];
        //     let reply = match socket.read(&mut buf).await {
        //         Ok(n) => {
        //             let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
        //             format!("Thanks for dialing in, {name}!\n")
        //         }
        //         Err(e) => {
        //             println!("socket error: {e:?}");
        //             return;
        //         }
        //     };

        //     if let Err(e) = socket.write_all(reply.as_bytes()).await {
        //         println!("socket error: {e:?}");
        //     }
        // });
    }
}

async fn join_example() {
    use anyhow::Result;
    use futures::future;
    use std::collections::HashMap;

    async fn size_of_page(url: &str) -> Result<usize> {
        let resp = reqwest::get(url).await?;
        Ok(resp.text().await?.len())
    }

    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}

// Dining Philosophers Async
use std::sync::Arc;
use tokio::time;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...
        println!("{} is trying to eat", &self.name);
        let _ = self.left_fork.lock().await;
        let _ = self.right_fork.lock().await;
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

async fn dining_philosophers() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    // Create philosophers
    let (tx, mut rx) = mpsc::channel(5);
    for i in 0..PHILOSOPHERS.len() {
        let p = Philosopher {
            name: PHILOSOPHERS[i].to_owned(),
            left_fork: Arc::clone(&forks[i]),
            right_fork: Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]),
            thoughts: tx.clone()
        };

        // Make them think and eat
        tokio::spawn(async move {
            for i in 0..5 {
                println!("Iteration {} for {}", i + 1, p.name.as_str());
                p.think().await;
                p.eat().await;
            }
        });
    }

    drop(tx);

    // Output their thoughts
    while let Some(thought) = rx.recv().await {
        println!("{thought}");
    }
}
