use serde_json::Value;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let _ = std::fs::remove_file(prex_core::SOCKET_PATH);

    let listener = UnixListener::bind(prex_core::SOCKET_PATH)?;

    println!("Daemon is running on {}", prex_core::SOCKET_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let packet_len: usize;
                let packet_string: String;
                {
                    // Get the packet length
                    let mut buffer = [0; size_of::<i32>()];
                    stream.read(&mut buffer)?;
                    packet_len = i32::from_ne_bytes(buffer) as usize;
                }

                {
                    // Get the packet itself
                    let mut buffer = vec![0; packet_len as usize];
                    stream.read(&mut buffer)?;

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
                            let packet_type: i8 =
                                (map.get("type").unwrap().as_i64().unwrap()) as i8;

                            if packet_type == prex_core::PACKET_SHUTDOWN {
                                println!("Daemon is shutting down...");
                                break;
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
                                            stream.write_all(&(0 as i32).to_ne_bytes())?;
                                        } else {
                                            // Send back the child's PID
                                            let child = child_res.ok().unwrap();
                                            stream.write_all(&child.id().to_ne_bytes())?;
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
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
