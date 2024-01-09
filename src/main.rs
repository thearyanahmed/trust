extern crate tun;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 1))
        .netmask((255, 255, 255, 0))
        .up();

    let mut dev = tun::create(&config).unwrap();
    let mut buf = [0; 4096];

    loop {
        let amount = dev.read(&mut buf).unwrap();
        println!("something is here{:?}", &buf[0..amount]);
    }
}
