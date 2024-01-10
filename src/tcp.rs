use std::io;
use etherparse::ip_number;
use etherparse::packet_filter::ElementFilter::No;
use crate::util::extract_ip_addresses;

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

        let (src, dst) = extract_ip_addresses(&ip_header);

        let ip = etherparse::Ipv4Header::new(
            syn_ack.header_len(),
            64,
            ip_number::TCP,
            dst,
            src,
        );

        syn_ack.checksum = syn_ack.calc_checksum_ipv4(&ip, &[])
            .expect("failed to calculate checksum");

        let unwritten = {
            let mut unwritten = &mut buf[..];
            let _ = ip.write(&mut unwritten);
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
        Ok(())
    }
}
