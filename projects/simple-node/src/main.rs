use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

struct VersionMessage {
    version: i32,
    services: u64,
    timestamp: i64,
}

fn send_version(stream: &mut TcpStream, msg: &VersionMessage) -> std::io::Result<()> {
    let payload = format!(
        "version: {}, services: {}, timestamp: {}",
        msg.version, msg.services, msg.timestamp
    );
    stream.write_all(payload.as_bytes())?;
    println!("Sent: {}", payload);
    Ok(())
}

fn handle_peer(mut stream: TcpStream) {
    let msg = VersionMessage {
        version: 70015,
        services: 0,
        timestamp: 1677654321,
    };
    if let Ok(()) =send_version(&mut stream, &msg) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => println!("Peer says: {}", String::from_utf8_lossy(&buffer[..n])),
            _ => println!("No reply"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8333")?;
    println!("Node on 127.0.0.1:8333...");
    for stream in listener.incoming() {
        let stream = stream?; // Unwrap Result here
        std::thread::spawn(|| handle_peer(stream));
    }
    Ok(())
}
