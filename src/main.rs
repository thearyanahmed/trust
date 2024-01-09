#[allow(dead_code)]
use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];

    eprintln!("entering capturing loop");

    loop {
        let read_bytes = nic.recv(&mut buf[..])?;

        eprintln!("read {} and bytes {:x?}", read_bytes, &buf[..read_bytes]);
    }
}
