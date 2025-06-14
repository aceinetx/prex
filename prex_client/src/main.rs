use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/prex.sock";

    let mut stream = UnixStream::connect(socket_path)?;

    stream.write_all(b"Hello from client!")?;

    let mut buffer = [0; prex_core::PACKET_LEN];
    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}
