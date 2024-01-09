#[allow(dead_code)]
use std::io;

// https://en.wikipedia.org/wiki/EtherType
const IP_V4_PROTO: u16 = 0x0800;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];

    eprintln!("entering capturing loop");

    loop {
        let read_bytes = nic.recv(&mut buf[..])?;

        // Frame format
        // https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/networking/tuntap.rst#n125
        let _ethernet_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let ethernet_protocol = u16::from_be_bytes([buf[2], buf[3]]);

        if ethernet_protocol != IP_V4_PROTO {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..read_bytes]) {
            Ok(packet) => {
                let src = packet.source_addr();
                let dest = packet.destination_addr();

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + packet.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{} -> {} {}b of TCP to port  ( {} )\n",
                            src,
                            dest,
                            p.slice().len(),
                            p.destination_port(),
                        );
                    }
                    Err(e) => {
                        eprintln!("could not parse TCP {:?}", e)
                    }
                }
            }
            Err(e) => {
                eprintln!("could not parse packet {:?}", e)
            }
        }
    }
}
