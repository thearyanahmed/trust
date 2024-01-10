use std::io;
use crate::util::extract_ip_addresses;

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

                let (src, dst) = extract_ip_addresses(&ip_header);

                let ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    6, // etherparse::IpNumber::Tcp,
                    dst,
                    src,
                );

                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    let _ = ip.write(&mut unwritten);
                    let _ = syn_ack.write(&mut unwritten);
                    unwritten.len()
                };

                nic.send(&buf[..unwritten])
            }
            State::Established => {
                eprintln!("established");
                Ok(0)
            }
            State::SynReceived => {
                eprintln!("syn received");
                Ok(0)
            }
        }
    }
}
