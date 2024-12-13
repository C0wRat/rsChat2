use std::net::TcpStream;
use std::io::{self, Write, Read};

/// Attempts to connect to a server with the given IP and port
pub fn connect_to_server(ip: &str, port: &str) -> io::Result<TcpStream> {
    let address = format!("{}:{}", ip, port);
    TcpStream::connect(&address)
}

/// Sends a message to the connected server
pub fn send_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    stream.write_all(message.as_bytes())
}

/// Receives a message from the server
pub fn receive_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}
