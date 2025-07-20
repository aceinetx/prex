pub mod client;

use serde_json::Value;
use std::{env, io};

fn print_usage() {
    println!("usage: prex [command] (args)");
    println!("            shutdown");
    println!("            info");
    println!("            exec      [name] (args...)");
}

fn print_usage_exit() {
    print_usage();
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    let client = client::Client::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage_exit();
    }

    if args[1] == "shutdown" {
        client.send_shutdown();
    } else if args[1] == "info" {
        let info = client.send_info();
        if info == Value::Null {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get info"));
        }

        let obj = info.as_object();
        match obj {
            Some(info_map) => {
                println!("prex daemon info:");
                if let Some(pid_o) = info_map.get("pid") {
                    if let Some(pid) = pid_o.as_number() {
                        println!("pid: {}", pid);
                    }
                }
            }

            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Recieved info is probably not an object",
                ));
            }
        };
    } else if args[1] == "exec" {
        if args.len() < 3 {
            print_usage_exit();
        }

        let mut argv = args;
        argv.drain(0..2);

        let result = client.send_exec(argv);
        if let Ok(pid) = result {
            println!("Success: process spawned with pid {}", pid);
        } else if let Err(error) = result {
            eprintln!("Error: {}", error);
        }
    } else {
        print_usage_exit();
    }

    Ok(())
}
