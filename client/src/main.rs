use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("Hello, client!");
    let mut stream = TcpStream::connect("127.0.0.1:33333")?;
    stream.write(&[1])?;

    Ok(())
}
