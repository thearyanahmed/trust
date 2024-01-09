# TRUST
Short for TCP-RUST, is an implementation of TCP (Transmission Control Protocol) in RUST. This project is an attempt to learn about how TCP works.

## Developing on Mac
This project was developed on a Apple Silicon M1. If you are trying something similar, you'll find out that
`tuntap` is no longer supported for macOS. So, I wrapped up the code in an ubuntu docker image to achieve the same.


## Quicklinks
- [TCP RFC 793](https://datatracker.ietf.org/doc/html/rfc793)
- [IP RFC 791](https://datatracker.ietf.org/doc/html/rfc791)
- [Linux Kernel > Networking > Tuntap](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/networking/tuntap.rst)
- [Ether type](https://en.wikipedia.org/wiki/EtherType)
## To consider
- [https://lib.rs/crates/tunio](https://lib.rs/crates/tunio)

## Quick Reference

### TCP Header Format
[https://datatracker.ietf.org/doc/html/rfc793#section-3.1](https://datatracker.ietf.org/doc/html/rfc793#section-3.1)

```txt
    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |          Source Port          |       Destination Port        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                        Sequence Number                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Acknowledgment Number                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |  Data |           |U|A|P|R|S|F|                               |
   | Offset| Reserved  |R|C|S|S|Y|I|            Window             |
   |       |           |G|K|H|T|N|N|                               |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |           Checksum            |         Urgent Pointer        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Options                    |    Padding    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                            TCP Header Format
```
