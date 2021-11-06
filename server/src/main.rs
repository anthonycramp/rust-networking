use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    println!("Hello, server!");

    let listener = TcpListener::bind("127.0.0.1:33333")?;

    for stream in listener.incoming() {
        println!("{:?}", stream?.peer_addr()?);
    }

    Ok(())
}
