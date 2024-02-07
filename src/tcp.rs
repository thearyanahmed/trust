use crate::util::extract_ip_addresses;
use etherparse::ip_number;
use etherparse::packet_filter::ElementFilter::No;
use std::io;

pub enum State {
    Closed,
    Listen,
    SynReceived,
    Established,
}

pub struct Connection {
    state: State,
    send: SendSequence,
    recv: ReceiveSequence,
    ip: etherparse::Ipv4Header,
}

pub struct SendSequence {
    una: u32,
    // send unacknowledged
    nxt: u32,
    // send nxt
    wnd: u16,
    // send window
    up: bool,
    // send urgent pointers
    wl1: usize,
    // segment sequence number used for last window update
    wl2: usize,
    // segment acknowledgment number used for last window update
    iss: u32,
}

pub struct ReceiveSequence {
    nxt: u32,
    // receive next
    wnd: u16,
    // receive window
    up: bool,
    // urgent pointer
    irs: u32, // initial sequence number
}

impl Connection {
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];

        eprintln!(
            "accepting connection \n {}:{} -> {}:{} {}b\n",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );

        if tcp_header.syn() {
            return Ok(None);
        }

        let (src, dst) = extract_ip_addresses(&ip_header);

        let iss = 0;

        let mut c = Connection {
            state: State::SynReceived,
            send: SendSequence {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: ReceiveSequence {
                irs: tcp_header.sequence_number(),
                nxt: tcp_header.sequence_number() + 1,
                wnd: tcp_header.window_size(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(0, 64, ip_number::TCP, dst, src),
        };

        let mut syn_ack = etherparse::TcpHeader::new(
            tcp_header.destination_port(),
            tcp_header.source_port(),
            c.send.iss,
            c.send.wnd,
        );

        syn_ack.acknowledgment_number = c.recv.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;

        c.ip.set_payload_len((syn_ack.header_len() + 0) as usize)
            .expect("failed to set ip payload length");

        let unwritten = {
            let mut unwritten = &mut buf[..];
            let _ = c.ip.write(&mut unwritten);
            let _ = syn_ack.write(&mut unwritten);
            unwritten.len()
        };

        nic.send(&buf[..unwritten])?;

        Ok(Some(c))
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {
        let ack_n = tcp_header.acknowledgment_number();

        if !is_between_wrapped(self.send.una, ack_n, self.send.nxt.wrapping_add(1)) {
            return Ok(());
        }

        let seq_n = tcp_header.sequence_number();
        let wnd_e = self.recv.nxt.wrapping_add(self.recv.wnd as u32);

        if data.len() == 0 && !tcp_header.syn() && !tcp_header.fin() {
            if self.recv.wnd == 0 {
                if seq_n != self.recv.nxt {
                    return Ok(());
                }
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seq_n, wnd_e) {
                return Ok(());
            }
        } else {
            if self.recv.wnd == 0 {
                return Ok(());
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seq_n, wnd_e)
                && !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seq_n + data.len() - 1, wnd_e)
            {
                return Ok(());
            }
        }

        let (src, dst) = extract_ip_addresses(&ip_header);

        eprintln!("received packets\n{:?} -> {:?}\n", src, dst);

        match self.state {
            State::Closed => {}
            State::Listen => {}
            State::SynReceived => {}
            State::Established => {
                eprintln!("state established, don't know what to do yet!");
            }
        }

        Ok(())
    }
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    use std::cmp::Ordering;

    match start.cmp(&x) {
        Ordering::Equal => return false,
        Ordering::Less => {
            if end >= start && end <= x {
                return false;
            }
        }
        Ordering::Greater => {
            if end < start && end > x {
            } else {
                return false;
            }
        }
    }

    true
}
