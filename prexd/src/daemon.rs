use serde_json::Value;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::thread;

pub struct Daemon {
    listener: UnixListener,
    listen: bool,
}

impl Daemon {
    pub fn new() -> Daemon {
        let _ = std::fs::remove_file(prex_core::SOCKET_PATH);
        return Daemon {
            listener: UnixListener::bind(prex_core::SOCKET_PATH).unwrap(),
            listen: true,
        };
    }

    fn process_packet(&mut self, stream: &mut UnixStream) {
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

        if parsed.is_err() {
            eprintln!("Failed to parse client's request");
        } else {
            if let Ok(Value::Object(map)) = parsed {
                // Do stuff with the packet
                if !map.contains_key("type") {
                    eprintln!("Client's request doesn't provide a type");
                } else {
                    let packet_type: i8 = (map.get("type").unwrap().as_i64().unwrap()) as i8;

                    if packet_type == prex_core::PACKET_SHUTDOWN {
                        println!("Daemon is shutting down...");
                        self.listen = false;
                    } else if packet_type == prex_core::PACKET_EXEC {
                        if let Some(Value::Array(args)) = map.get("args") {
                            let exe_name: String;
                            let mut argv: Vec<String> = args
                                .iter()
                                .filter_map(|arg| arg.as_str().map(String::from))
                                .collect();

                            if argv.len() == 0 {
                                eprintln!("Argv is empty!");
                            } else {
                                exe_name = argv.get(0).unwrap().to_string();
                                argv.drain(0..1);

                                // Execute the process
                                let child_res = Command::new(exe_name).args(&argv).spawn();
                                if child_res.is_err() {
                                    eprintln!(
                                        "Failed to spawn a child: {}",
                                        child_res.err().unwrap()
                                    );
                                    // Send back zero indicating an error
                                    let _ = stream.write_all(&(0 as i32).to_ne_bytes());
                                } else {
                                    // Send back the child's PID
                                    let mut child = child_res.ok().unwrap();
                                    let _ = stream.write_all(&child.id().to_ne_bytes());

                                    let child_id = child.id();
                                    thread::spawn(move || {
                                        let _ = child.wait();
                                        println!("Child process {} has exited", child_id);
                                    });
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("Parsed value is not an object");
            }
        }
    }

    pub fn listen(&mut self) {
        self.listen = true;
        println!("Daemon is running on {}", prex_core::SOCKET_PATH);

        let listener = self.listener.try_clone().unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    self.process_packet(&mut stream);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }

            if !self.listen {
                break;
            };
        }
    }
}
