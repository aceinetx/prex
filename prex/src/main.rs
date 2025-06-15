use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn print_usage() {
    println!("usage: prex [command] (args)");
    println!("            shutdown");
    println!("            exec      [name] (args...)");
}

fn print_usage_exit() {
    print_usage();
    std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage_exit();
    }

    if args[1] == "shutdown" {
        let mut stream = UnixStream::connect(prex_core::SOCKET_PATH)?;

        let packet = prex_core::construct_shutdown_packet();
        stream.write_all(&(packet.len() as i32).to_ne_bytes())?;

        stream.write_all(packet.as_bytes())?;

        // TODO: maybe implement some mechanism that returns an error (if we would even have it)
    } else if args[1] == "exec" {
        if args.len() < 3 {
            print_usage_exit();
        }

        let mut argv = args;
        argv.drain(0..2);

        let mut stream = UnixStream::connect(prex_core::SOCKET_PATH)?;

        let packet = prex_core::construct_exec_packet(argv);
        stream.write_all(&(packet.len() as i32).to_ne_bytes())?;

        stream.write_all(packet.as_bytes())?;

        {
            let mut buffer = [0; size_of::<i32>()];
            stream.read(&mut buffer)?;
            let pid = i32::from_ne_bytes(buffer);
            if pid == 0 {
                eprintln!(
                    "Error: pid == 0 (no such executable? if you have access to the daemon's stderr, you should check out the error)"
                );
            } else {
                println!("Success: process spawned with pid {}", pid);
            }
        }
    } else {
        print_usage_exit();
    }

    Ok(())
}
