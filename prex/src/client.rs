use serde_json::{Map, Value};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub struct Client {}

impl Client {
    pub fn new() -> Client {
        return Client {};
    }

    pub fn send_info(&self) -> Value {
        let mut stream = UnixStream::connect(prex_core::SOCKET_PATH).unwrap();

        let packet = prex_core::construct_info_packet();
        stream
            .write_all(&(packet.len() as i32).to_ne_bytes())
            .unwrap();

        stream.write_all(packet.as_bytes()).unwrap();

        let packet_len: usize;
        let packet_string: String;
        {
            // Get the packet length
            let mut buffer = [0; size_of::<i32>()];
            let _ = stream.read(&mut buffer);
            packet_len = i32::from_ne_bytes(buffer) as usize;
        }

        {
            // Get the packet itself
            let mut buffer = vec![0; packet_len as usize];
            let _ = stream.read(&mut buffer);

            packet_string = String::from_utf8_lossy(&buffer).to_string();
        }

        let parsed = serde_json::from_str::<Value>(packet_string.as_str());

        if !parsed.is_err() {
            if let Ok(Value::Object(map)) = parsed {
                return Value::Object(map);
            }
        }
        return serde_json::Value::Null;
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
