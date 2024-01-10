use std::io;
use std::prelude::*;

use etherparse::IpHeader;

pub enum State {
    Closed,
    Listen,
    SynReceived,
    Established,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        // let mut unwritten = &mut buf[..];
        eprintln!(
            "{}:{} -> {}:{} {}b\n",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );

        match *self {
            State::Closed => return Ok(0),
            State::Listen => {
                if tcp_header.syn() {
                    return Ok(0);
                }

                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0,
                    0,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                let ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    6, //,etherparse::IpNumber::Tcp,
                    [
                        ip_header.destination()[0],
                        ip_header.destination()[1],
                        ip_header.destination()[2],
                        ip_header.destination()[3],
                    ],
                    [
                        ip_header.source()[0],
                        ip_header.source()[1],
                        ip_header.source()[2],
                        ip_header.source()[3],
                    ],
                );

                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    let _ = ip.write(&mut unwritten);
                    let _ = syn_ack.write(&mut unwritten);
                    unwritten.len()
                };

                nic.send(&buf[..unwritten])
            }
            State::Established => Ok(0),
            State::SynReceived => Ok(0),
        }
    }
}
