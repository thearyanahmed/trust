#[allow(dead_code)]
use std::io;

use std::io::{Read, Write};
// use tunio::platform::utun::Driver;
use tunio::traits::{DriverT, InterfaceT};
use tunio::{DefaultDriver, DefaultInterface};

fn main() -> io::Result<()> {
    let _ = trust();
    // tunio();

    Ok(())
}
fn tunio() {
    // DefaultDriver is an alias for a supported driver for current platform.
    // It may be not optimal for your needs (for example, it can lack support of TAP),
    // but it will work in some cases. If you need another driver, then import and use it instead.
    let mut driver = DefaultDriver::new().unwrap();
    // Preparing configuration for new interface. We use `Builder` pattern for this.
    let if_config = DefaultDriver::if_config_builder()
        .name("tun0".to_string())
        .build()
        .unwrap();

    // Then, we create the interface using config and start it immediately.
    let mut interface =
        DefaultInterface::new_up(&mut driver, if_config).expect("failed to create interface");

    // The interface is created and running.

    // Write to interface using Write trait
    let buf = [0u8; 4096];
    let _ = interface.write(&buf);

    // Read from interface using Read trait
    let mut mut_buf = [0u8; 4096];
    let _ = interface.read(&mut mut_buf);
}

fn trust() -> io::Result<()> {
    println!("Init");

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];

    eprintln!("entering loop, check tun0 dev in another devices");

    loop {
        let read_bytes = nic.recv(&mut buf[..])?;

        eprintln!("read {} and bytes {:x?}", read_bytes, &buf[..read_bytes]);
    }
}
