use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub struct Client {}

impl Client {
    pub fn new() -> Client {
        return Client {};
    }

    pub fn send_shutdown(&self) {
        let mut stream = UnixStream::connect(prex_core::SOCKET_PATH).unwrap();

        let packet = prex_core::construct_shutdown_packet();
        stream
            .write_all(&(packet.len() as i32).to_ne_bytes())
            .unwrap();

        stream.write_all(packet.as_bytes()).unwrap();

        // TODO: maybe implement some mechanism that returns an error (if we would even have it)
    }

    pub fn send_exec(&self, argv: Vec<String>) -> Result<i32, &'static str> {
        let mut stream = UnixStream::connect(prex_core::SOCKET_PATH).unwrap();

        let packet = prex_core::construct_exec_packet(argv);
        stream
            .write_all(&(packet.len() as i32).to_ne_bytes())
            .unwrap();

        stream.write_all(packet.as_bytes()).unwrap();

        {
            let mut buffer = [0; size_of::<i32>()];
            stream.read(&mut buffer).unwrap();
            let pid = i32::from_ne_bytes(buffer);
            if pid == 0 {
                // TODO: maybe make the server respond with an error?
                return Err(
                    "pid == 0 (no such executable? if you have access to the daemon's stderr, you should check out the error)",
                );
            } else {
                return Ok(pid);
            }
        }
    }
}
