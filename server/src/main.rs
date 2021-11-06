use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    println!("{:?}", stream.peer_addr()?);
    let mut buf: Vec<u8> = vec![];
    stream.read(&mut buf)?;
    println!("Received message: {:?}", from_utf8(&buf));
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, server!");

    let listener = TcpListener::bind("127.0.0.1:33333")?;

    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }

    Ok(())
}
