use std::net::TcpStream;
use std::io::{prelude::*,BufReader,Write};


pub fn tcp_sender(host: &str, msg: &[u8]) -> Result<(), std::io::Error>{
    match TcpStream::connect(host) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            stream.write(msg)?;
            println!("Sent Hello, awaiting reply...");
            let mut reader = BufReader::new(&stream);
            let mut buffer: Vec<u8> = Vec::new();
            reader.read_until(b'\n',&mut buffer)?;
            println!("FF");
            println!("{}",std::str::from_utf8(&buffer).unwrap());
        },
        Err(e) => {
            println!("UNABLE TO CONNECT");
        }
    }
    Ok(())
}