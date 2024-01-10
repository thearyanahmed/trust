use etherparse::{Ipv4HeaderSlice};

pub fn extract_ip_addresses(ip_header: &Ipv4HeaderSlice) -> ([u8; 4], [u8; 4]) {
    let destination_ip = [
        ip_header.destination()[0],
        ip_header.destination()[1],
        ip_header.destination()[2],
        ip_header.destination()[3],
    ];

    let source_ip = [
        ip_header.source()[0],
        ip_header.source()[1],
        ip_header.source()[2],
        ip_header.source()[3],
    ];

    (source_ip, destination_ip)
}