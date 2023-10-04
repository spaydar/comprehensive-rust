use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
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
