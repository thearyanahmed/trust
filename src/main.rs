#[allow(dead_code)]
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;
use crate::tcp::State;

mod tcp;
mod util;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Qaud {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

// https://en.wikipedia.org/wiki/EtherType
const IP_V4_PROTO: u16 = 0x0800;
const TCP_PROTOCOL: u8 = 0x06;

fn main() -> io::Result<()> {
    let mut connections: HashMap<Qaud, tcp::Connection> = Default::default();

    let mut nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0u8; 1504];

    eprintln!("entering capturing loop");

    loop {
        let read_bytes = nic.recv(&mut buf[..])?;

        // Frame format
        // https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/networking/tuntap.rst#n125
        // let _ethernet_flags = u16::from_be_bytes([buf[0], buf[1]]);
        // let ethernet_protocol = u16::from_be_bytes([buf[2], buf[3]]);
        //
        // if ethernet_protocol != IP_V4_PROTO {
        //     continue;
        // }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[..read_bytes]) {
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dst: Ipv4Addr = ip_header.destination_addr();

                if ip_header.protocol() != TCP_PROTOCOL {
                    continue; // Not TCP
                }

                match etherparse::TcpHeaderSlice::from_slice(
                    &buf[ip_header.slice().len()..read_bytes],
                ) {
                    Ok(tcp_header) => {
                        use std::collections::hash_map::Entry;
                        let data = ip_header.slice().len() + tcp_header.slice().len();

                        match (connections
                            .entry(Qaud {
                                src: (src, tcp_header.source_port()),
                                dst: (dst, tcp_header.destination_port()),
                            })) {
                            Entry::Occupied(mut c) => {
                                c.get_mut().on_packet(&mut nic, ip_header, tcp_header, &buf[data..read_bytes])?;
                            }
                            Entry::Vacant(mut e) => {
                                if let Some(c) = tcp::Connection::accept(&mut nic, ip_header, tcp_header, &buf[data..read_bytes])? {
                                    e.insert(c);
                                }
                            }
                        }
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
