use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

fn request_block(stream: &mut TcpStream, block_hash: &str) -> std::io::Result<()> {
    let msg = format!("getblock: {}", block_hash);
    stream.write_all(msg.as_bytes())?;
    println!("Requested block: {}", block_hash);
    Ok(())
}

fn handle_peer(mut stream: TcpStream) {
    let version = VersionMessage {
        version: 70015,
        services: 0,
        timestamp: 1677654321,
    };
    if send_version(&mut stream, &version).is_ok() {
        let mut buffer = [0; 1024];
        if let Ok(n) = stream.read(&mut buffer) {
            if n > 0 {
                println!("Peer says: {}", String::from_utf8_lossy(&buffer[..n]));
                request_block(&mut stream, "0000abc...").ok();
                if let Ok(n) = stream.read(&mut buffer) {
                    println!("Block data: {}", String::from_utf8_lossy(&buffer[..n]));
                }
            }
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
