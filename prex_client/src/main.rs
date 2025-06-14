use prex_core::add;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    println!("{}", add(2, 3));

    let socket_path = "/tmp/prex.sock";

    let mut stream = UnixStream::connect(socket_path)?;

    stream.write_all(b"Hello from client!")?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}
