pub mod daemon;

fn main() -> std::io::Result<()> {
    let mut daemon = daemon::Daemon::new();
    daemon.listen();

    Ok(())
}
