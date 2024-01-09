use std::io;
use std::time::Duration;

// This is a placeholder process to keep the container alive
fn main() -> io::Result<()> {
    let mut i = 1;

    loop {
        println!("running every 5000 ms | {}", i);
        i = i + 1;
        std::thread::sleep(Duration::from_millis(5000));
    }
}
