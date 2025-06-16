pub mod client;

use std::env;

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
    let client = client::Client::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage_exit();
    }

    if args[1] == "shutdown" {
        client.send_shutdown();
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
