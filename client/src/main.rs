use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("Hello, client!");
    let mut stream = TcpStream::connect("127.0.0.1:33333")?;
    println!("Local address: {:?}", stream.local_addr()?);
    println!("Peer address: {:?}", stream.peer_addr()?);
    let message = "hello, server. I'm client";
    stream.write(message.as_bytes())?;

    Ok(())
}
