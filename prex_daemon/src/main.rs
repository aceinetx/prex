use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/prex.sock";

    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;

    println!("Server listening on {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");

                let mut buffer = [0; prex_core::PACKET_LEN];
                let bytes_read = stream.read(&mut buffer)?;
                println!(
                    "Received: {}",
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );

                stream.write_all(b"Hello from server!")?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
